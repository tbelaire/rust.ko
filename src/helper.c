#include "helper.h"

unsigned my_scsi_bufflen(struct scsi_cmnd *cmd) {
	return scsi_bufflen(cmd);
}

void my_scsi_set_resid(struct scsi_cmnd *cmd, int resid) {
	scsi_set_resid(cmd, resid);
}

int my_scsi_get_resid(struct scsi_cmnd *cmd) {
	return scsi_get_resid(cmd);
}

unsigned int my_cpu_to_le32(unsigned int x) {
	return cpu_to_le32(x);
}

unsigned int my_le32_to_cpu(unsigned int x) {
	return le32_to_cpu(x);
}
