#include <linux/sched.h>
#include <linux/gfp.h>
#include <linux/errno.h>
#include <linux/export.h>

#include <linux/usb/quirks.h>

#include <scsi/scsi.h>
#include <scsi/scsi_eh.h>
#include <scsi/scsi_device.h>

#include "usb.h"
#include "transport.h"
#include "protocol.h"
#include "scsiglue.h"
#include "debug.h"

#include <linux/blkdev.h>
#include "scsi/sd.h"

// To avoid double declaring these, only include in helper.c
const unsigned int CONST_US_BULK_CB_SIGN = US_BULK_CB_SIGN;
const unsigned int CONST_US_BULK_CB_WRAP_LEN = US_BULK_CB_WRAP_LEN;

const unsigned int CONST_US_BULK_CS_SIGN = US_BULK_CS_SIGN;
const unsigned int CONST_US_BULK_CS_WRAP_LEN = US_BULK_CS_WRAP_LEN;

const unsigned char CONST_US_BULK_FLAG_IN = US_BULK_FLAG_IN;

const unsigned char CONST_US_BULK_STAT_OK = US_BULK_STAT_OK;
const unsigned char CONST_US_BULK_STAT_FAIL = US_BULK_STAT_FAIL;
const unsigned char CONST_US_BULK_STAT_PHASE = US_BULK_STAT_PHASE;


const unsigned long int CONST_US_FL_BULK_IGNORE_TAG = US_FL_BULK_IGNORE_TAG;
const unsigned long int CONST_US_FL_IGNORE_RESIDUE = US_FL_IGNORE_RESIDUE;
const unsigned long int CONST_US_FL_SCM_MULT_TARG = US_FL_SCM_MULT_TARG;
const unsigned long int CONST_US_FL_BULK32 = US_FL_BULK32;

const int CONST_USB_STOR_XFER_GOOD = USB_STOR_XFER_GOOD;
const int CONST_USB_STOR_XFER_STALLED = USB_STOR_XFER_STALLED;
const int CONST_USB_STOR_XFER_ERROR = USB_STOR_XFER_ERROR;
const int CONST_USB_STOR_XFER_LONG = USB_STOR_XFER_LONG;
const int CONST_USB_STOR_XFER_SHORT = USB_STOR_XFER_SHORT;

const int CONST_USB_STOR_TRANSPORT_GOOD = USB_STOR_TRANSPORT_GOOD;
const int CONST_USB_STOR_TRANSPORT_ERROR = USB_STOR_TRANSPORT_ERROR;
const int CONST_USB_STOR_TRANSPORT_FAILED = USB_STOR_TRANSPORT_FAILED;
const int CONST_USB_STOR_TRANSPORT_NO_SENSE = USB_STOR_TRANSPORT_NO_SENSE;

const unsigned long int CONST_US_FL_GO_SLOW = US_FL_GO_SLOW;


const unsigned char CONST_INQUIRY = INQUIRY;
const unsigned char CONST_READ_CAPACITY = READ_CAPACITY;

unsigned my_scsi_bufflen(struct scsi_cmnd *cmd);
void my_scsi_set_resid(struct scsi_cmnd *cmd, int resid);
int my_scsi_get_resid(struct scsi_cmnd *cmd);

unsigned int my_cpu_to_le32(unsigned int);
unsigned int my_le32_to_cpu(unsigned int);
