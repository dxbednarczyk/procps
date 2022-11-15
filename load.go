package main

/*
#cgo LDFLAGS: -lprocps
#include <proc/sysinfo.h>
*/
import "C"

type LoadInfo struct {
	Av1  float64 `json:"avg_one_minute"`
	Av5  float64 `json:"avg_five_minutes"`
	Av15 float64 `json:"avg_fifteen_minutes"`
}

func GetLoadInfo() LoadInfo {
	var av1 C.double
	var av5 C.double
	var av15 C.double

	C.loadavg(&av1, &av5, &av15)

	return LoadInfo{
		Av1:  float64(av1),
		Av5:  float64(av5),
		Av15: float64(av15),
	}
}
