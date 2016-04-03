#![feature(core_str_ext, custom_attribute, lang_items)]
#![no_std]

#[macro_use]
extern crate linux_std as std;

use std::os::raw::*;
use std::os::kernel;
use std::os::kernel:: *;

use core::ptr::{null, null_mut};

// Defines various language items that need to be around
mod lang;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust");
}

#[no_mangle]
pub unsafe fn rust_Bulk_transport(srb: *mut Struct_scsi_cmnd,
                           us:  *mut Struct_us_data) -> c_int {
    let bcb = (*us).iobuf as *mut Struct_bulk_cb_wrap;
    let bcs = (*us).iobuf as *mut Struct_bulk_cs_wrap;
    let transfer_length = my_scsi_bufflen(srb);
    let mut residue: c_uint;
    let mut result: c_int;
    let mut fake_sense: c_int = 0;
    let mut cswlen: c_uint = 0;
    let mut cbwlen: c_uint = CONST_US_BULK_CB_WRAP_LEN;

    /* Take care of BULK32 devices; set extra byte to 0 */
    if (*us).fflags & CONST_US_FL_BULK32 != 0 {
        cbwlen = 32;
        *(*us).iobuf.offset(31) = 0;
    }

    /* set up the command wrapper */
    (*bcb).Signature = my_cpu_to_le32(CONST_US_BULK_CB_SIGN);
    (*bcb).DataTransferLength = my_cpu_to_le32(transfer_length);
    (*bcb).Flags = if (*srb).sc_data_direction as usize ==
        Enum_dma_data_direction::DMA_FROM_DEVICE as usize {
            CONST_US_BULK_FLAG_IN
        } else { 0 } ;
    (*us).tag += 1;
    (*bcb).Tag = (*us).tag;
    (*bcb).Lun = (*(*srb).device).lun as u8;
    if (*us).fflags & CONST_US_FL_SCM_MULT_TARG != 0 {
        (*bcb).Lun |= ((*(*srb).device).id << 4) as u8;
    }
    (*bcb).Length = (*srb).cmd_len as u8;

    /* copy the command payload */
    kernel::memset(&mut (*bcb).CDB as *mut _ as *mut c_void,
        0,
        core::mem::size_of::<[__u8; 16usize]>() as u64);// sizeof(bcb->CDB));
    kernel::memcpy(&mut (*bcb).CDB as *mut _ as *mut c_void,
                   &mut (*srb).cmnd as *mut _ as *mut c_void,
                   (*bcb).Length as u64);
    /* send it to out endpoint */
    println!("Bulk Command S 0x{:x} T 0x{:x} L {} F {} Trg {} LUN {} CL {}",
                 my_le32_to_cpu((*bcb).Signature), (*bcb).Tag,
                 my_le32_to_cpu((*bcb).DataTransferLength), (*bcb).Flags,
                 ((*bcb).Lun >> 4), ((*bcb).Lun & 0x0F),
                 (*bcb).Length);

    result = usb_stor_bulk_transfer_buf(us, (*us).send_bulk_pipe,
                                        bcb as *mut c_void,
                                        cbwlen,
                                        null_mut::<u32>());
    println!("Bulk command transfer result={}", result);
    if result != CONST_USB_STOR_XFER_GOOD {
        return CONST_USB_STOR_TRANSPORT_ERROR;
    }

    /* DATA STAGE */
    /* send/receive data payload, if there is any */

    /* Some USB-IDE converter chips need a 100us delay between the
     * command phase and the data phase.  Some devices need a little
     * more than that, probably because of clock rate inaccuracies. */
    if (*us).fflags & CONST_US_FL_GO_SLOW != 0 {
        usleep_range(125, 150);
    }

    let mut goto_skipped_data_phase = false;
    if transfer_length != 0 {
        let pipe: c_uint = if (*srb).sc_data_direction as usize ==
            Enum_dma_data_direction::DMA_FROM_DEVICE as usize {
            (*us).recv_bulk_pipe
        } else {
            (*us).send_bulk_pipe };
        result = usb_stor_bulk_srb(us, pipe, srb);
        println!("Bulk data transfer result 0x{:x}", result);
        if result == CONST_USB_STOR_XFER_ERROR {
            return CONST_USB_STOR_TRANSPORT_ERROR;
        }

        /* If the device tried to send back more data than the
         * amount requested, the spec requires us to transfer
         * the CSW anyway.  Since there's no point retrying the
         * the command, we'll return fake sense data indicating
         * Illegal Request, Invalid Field in CDB.
         */
        if result == CONST_USB_STOR_XFER_LONG {
            fake_sense = 1;
        }

        /*
         * Sometimes a device will mistakenly skip the data phase
         * and go directly to the status phase without sending a
         * zero-length packet.  If we get a 13-byte response here,
         * check whether it really is a CSW.
         */
        if result == CONST_USB_STOR_XFER_SHORT &&

            (*srb).sc_data_direction as usize
                == Enum_dma_data_direction:: DMA_FROM_DEVICE  as usize &&

                transfer_length as i32 - my_scsi_get_resid(srb)
                == CONST_US_BULK_CS_WRAP_LEN as i32 {

            let mut sg: *mut Struct_scatterlist = null_mut();
            let mut offset: c_uint = 0;

            if (usb_stor_access_xfer_buf(
                    bcs as *mut u8,
                    CONST_US_BULK_CS_WRAP_LEN, srb,
                    &mut sg,
                    &mut offset, Enum_xfer_buf_dir::FROM_XFER_BUF)
                == CONST_US_BULK_CS_WRAP_LEN &&
                (*bcs).Signature == my_cpu_to_le32(CONST_US_BULK_CS_SIGN)) {

                println!("Device skipped data phase");
                my_scsi_set_resid(srb, transfer_length as i32);
                goto_skipped_data_phase = true;
            }
        }
    }
    if !goto_skipped_data_phase {
        /* See flow chart on pg 15 of the Bulk Only Transport spec for
         * an explanation of how this code works.
         */

        /* get CSW for device status */
        println!("Attempting to get CSW...");
        result = usb_stor_bulk_transfer_buf(
            us, (*us).recv_bulk_pipe,
            bcs as *mut c_void, CONST_US_BULK_CS_WRAP_LEN,
            &mut cswlen);

        /* Some broken devices add unnecessary zero-length packets to the
         * end of their data transfers.  Such packets show up as 0-length
         * CSWs.  If we encounter such a thing, try to read the CSW again.
         */
        if result == CONST_USB_STOR_XFER_SHORT && cswlen == 0 {
            println!("Received 0-length CSW; retrying...");
            result = usb_stor_bulk_transfer_buf(
                us, (*us).recv_bulk_pipe,
                bcs as *mut c_void, CONST_US_BULK_CS_WRAP_LEN,
                &mut cswlen);
        }

        /* did the attempt to read the CSW fail? */
        if result == CONST_USB_STOR_XFER_STALLED {

            /* get the status again */
            println!("Attempting to get CSW (2nd try)...");
            result = usb_stor_bulk_transfer_buf(
                us, (*us).recv_bulk_pipe,
                bcs as *mut c_void, CONST_US_BULK_CS_WRAP_LEN,
                null_mut());
        }

        /* if we still have a failure at this point, we're in trouble */
        println!("Bulk status result = {}", result);
        if result != CONST_USB_STOR_XFER_GOOD {
            return CONST_USB_STOR_TRANSPORT_ERROR;
        }

    }
    // label skipped_data_phase:
    /* check bulk status */
    residue = my_le32_to_cpu((*bcs).Residue);
    println!("Bulk Status S 0x{:x} T 0x{:x} R {} Stat 0x{:x}",
             my_le32_to_cpu((*bcs).Signature), (*bcs).Tag,
             residue, (*bcs).Status);
    if !((*bcs).Tag == (*us).tag ||
         ((*us).fflags & CONST_US_FL_BULK_IGNORE_TAG != 0)) ||
        (*bcs).Status > CONST_US_BULK_STAT_PHASE as u8 {

        println!("Bulk logical error");
        return CONST_USB_STOR_TRANSPORT_ERROR;
    }
    /* Some broken devices report odd signatures, so we do not check them
     * for validity against the spec. We store the first one we see,
     * and check subsequent transfers for validity against this signature.
     */
    if (*us).bcs_signature != 0 {
        (*us).bcs_signature = (*bcs).Signature;
        if (*us).bcs_signature != my_cpu_to_le32(CONST_US_BULK_CS_SIGN) {
            println!("Learnt BCS signature 0x{:x}",
                     my_le32_to_cpu((*us).bcs_signature));
        }
    } else if ((*bcs).Signature != (*us).bcs_signature) {
        println!("Signature mismatch: got {:x}, expecting {:x}",
                 my_le32_to_cpu((*bcs).Signature),
                 my_le32_to_cpu((*us).bcs_signature));
        return CONST_USB_STOR_TRANSPORT_ERROR;
    }

    /* try to compute the actual residue, based on how much data
     * was really transferred and what the device tells us */
    if residue != 0 && ((*us).fflags & CONST_US_FL_IGNORE_RESIDUE != 0) {

        /* Heuristically detect devices that generate bogus residues
         * by seeing what happens with INQUIRY and READ CAPACITY
         * commands.
         */
        if (*bcs).Status == CONST_US_BULK_STAT_OK as u8 &&
            my_scsi_get_resid(srb) == 0 &&
                (((*(*srb).cmnd) == CONST_INQUIRY &&
                  transfer_length == 36) ||
                 ((*(*srb).cmnd) == CONST_READ_CAPACITY &&
                  transfer_length == 8)) {

            (*us).fflags |= CONST_US_FL_IGNORE_RESIDUE;

        } else {
            residue = core::cmp::min(residue, transfer_length);
            my_scsi_set_resid(srb, core::cmp::max(
                    my_scsi_get_resid(srb),
                    residue as c_int));
        }
    }

    /* based on the status code, we report good or bad */
    let status = (*bcs).Status;
    return if status == CONST_US_BULK_STAT_OK {
        /* device babbled -- return fake sense data */
        if (fake_sense != 0) {
            memcpy(
                (*srb).sense_buffer as *mut _ as *mut _,
                &usb_stor_sense_invalidCDB as *const _ as *const _,
            core::mem::size_of::<[::std::os::raw::c_uchar; 18usize]>() as u64);
            CONST_USB_STOR_TRANSPORT_NO_SENSE
        } else {
            /* command good -- note that data could be short */
            CONST_USB_STOR_TRANSPORT_GOOD
        }
    } else if status == CONST_US_BULK_STAT_FAIL {
        /* command failed */
        CONST_USB_STOR_TRANSPORT_FAILED
    } else if status == CONST_US_BULK_STAT_PHASE {
        /* phase error -- note that a transport reset will be
         * invoked by the invoke_transport() function
         */
        CONST_USB_STOR_TRANSPORT_ERROR
    } else {
        /* we should never get here,
         * but if we do, we're in trouble */
        CONST_USB_STOR_TRANSPORT_ERROR
    };
}
