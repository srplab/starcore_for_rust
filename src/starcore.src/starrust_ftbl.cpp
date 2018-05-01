#include "vsopenapi.h"

extern "C" SRPDLLEXPORT void *StarRust_GetFunctionTbl_FromContol(class ClassOfSRPControlInterface *SRPControl)
{
	return SRPControl -> GetCFunctionTable();
}
