package main

/*
#cgo LDFLAGS: -lprocps
#include <proc/sysinfo.h>
*/
import "C"

type MemInfo struct {
	Total  int64 `json:"total"`
	Used   int64 `json:"used"`
	Free   int64 `json:"free"`
	Shared int64 `json:"shared"`
}

func GetMemInfo() MemInfo {
	C.meminfo()

	return MemInfo{
		Total:  int64(C.kb_main_total),
		Used:   int64(C.kb_main_used),
		Free:   int64(C.kb_main_free),
		Shared: int64(C.kb_main_shared),
	}
}
