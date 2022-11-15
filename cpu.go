package main

/*
#cgo LDFLAGS: -lprocps
#include <proc/sysinfo.h>
*/
import "C"

type CPUInfo struct {
	Hertz     int64 `json:"hz"`
	CPUs      int   `json:"cpus"`
	PageBytes int   `json:"page_bytes"`
}

func GetCPUInfo() CPUInfo {
	C.cpuinfo()

	return CPUInfo{
		Hertz:     int64(C.Hertz),
		CPUs:      int(C.smp_num_cpus),
		PageBytes: int(C.page_bytes),
	}
}
