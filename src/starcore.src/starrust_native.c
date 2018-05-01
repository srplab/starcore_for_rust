#include "vsopenapi_c.h"
#include "vsopenapi_c_stub.h"
#include "starrust_native.h"

static VS_HANDLE hDllInstance;
static VSCore_RegisterCallBackInfoProc RegisterCallBackInfoProc = NULL;
static VSCore_UnRegisterCallBackInfoProc UnRegisterCallBackInfoProc = NULL;
static VSCore_InitProc VSInitProc = NULL;
static VSCore_TermProc VSTermProc = NULL;
static VSCore_TermExProc VSTermExProc = NULL;
static VSCore_HasInitProc HasInitProc = NULL;
static VSCore_QueryControlInterfaceProc QueryControlInterfaceProc = NULL;
static SRPControl_GetCFunctionTable_Proc Control_GetCFunctionTable;

void *StarRust_SRPControlInterface = NULL;
void *StarRust_SRPCoreShellInterface = NULL;

VS_INT32 StarRust_ScriptInterfaceIndex;

VS_BOOL StarRust_ModuleInitFlag = VS_FALSE;
static VS_BOOL ModuleLoadFlag = VS_FALSE;
VS_BOOL StarRust_ModuleLoadFromStarcore = VS_FALSE;

VS_CHAR  StarRust_g_ShareLibraryPath[512] = {0};
VS_CHAR  StarRust_g_CoreLibraryPath[512] = {0};
VS_CHAR  StarRust_g_CoreOperationPath[512] = {0};

/*-------------callback from rust begin----*/
extern void RustScriptTerm();

extern VS_UWORD RustMsgCallBack(VS_ULONG ServiceGroupID, VS_ULONG uMsg, VS_UWORD wParam, VS_UWORD lParam, VS_BOOL *IsProcessed, VS_UWORD Para);
extern void RustSRPDispatchRequestCallBack();
extern void RustFreeScriptObject(VS_UWORD RefItem);
extern void RustObjectFreeNotify(void *Object,VS_UWORD RefItem);
extern void RustObjectIDChangeNotify(void *Object, VS_UWORD Para, VS_UUID *NewObjectID);
extern VS_INT32 RustSRPObject_ScriptCallBack(void *L);

#if defined(STAR_RUST_USERAW)
extern VS_BOOL RustRawContext_LuaFuncFilter(void *object, void *ForWhichObject, VS_CHAR *FuncName, VS_UWORD Para);
extern VS_INT32 RustRawContext_GeneralFunction(void *L);

extern VS_CHAR *RustVSScript_GetRawContextType(VS_UWORD Para, VS_ULONG ServiceGroupIndex, void *object);
extern VS_PARAPKGPTR RustVSScript_RawToParaPkg( VS_UWORD Para, VS_ULONG ServiceGroupIndex,void *Object );
extern VS_BOOL RustVSScript_AttachRawContext(VS_UWORD Para, VS_ULONG ServiceGroupIndex, void *object, VS_CHAR *ContextName, VS_BOOL IsClass, VS_CHAR *ContextInfo);

extern VS_BOOL RustRawContext_RegGetValue(void *Object, void *ForWhichObject, VS_CHAR *Name, VS_UWORD Para, VS_BOOL GetAllRawAttributeFlag);
extern VS_BOOL RustRawContext_RegSetValue(void *Object, void *ForWhichObject, VS_CHAR *Name, VS_INT32 Index, VS_UWORD Para);
#endif

/*--this function is used to init rust from starcore--*/
extern VS_BOOL RustVSScript_InitRaw(VS_UWORD Para, void *BasicSRPInterface, void *SRPInterface);

extern void RustInitFromStarCore();
/*-------------callback from rust end------*/

static void starrust_starcore_ScriptTerm()
{
	if( StarRust_SRPControlInterface != NULL )
		Star_SRPControl_ClearScriptObject(StarRust_SRPControlInterface,starrust_FreeScriptObject, 0 );
	if( StarRust_ModuleInitFlag == VS_FALSE ){

		RustScriptTerm();
		
		ModuleLoadFlag = VS_FALSE;
		StarRust_ModuleInitFlag = VS_FALSE;
		if( StarRust_SRPCoreShellInterface != NULL )
			Star_SRPCS_Release(StarRust_SRPCoreShellInterface);
		StarRust_SRPCoreShellInterface = NULL;

		return;
	}
	
	RustScriptTerm();

	if( StarRust_SRPCoreShellInterface != NULL )
		Star_SRPCS_Release(StarRust_SRPCoreShellInterface);
	StarRust_SRPCoreShellInterface = NULL;

	ModuleLoadFlag = VS_FALSE;
	StarRust_ModuleInitFlag = VS_FALSE;
		
	if( StarRust_SRPControlInterface != NULL )
		Star_SRPControl_Release(StarRust_SRPControlInterface);
	StarRust_SRPControlInterface = NULL;

	return;
}

#if defined(STAR_RUST_USERAW)
static VS_CHAR *starrust_VSScript_GetRawContextType(VS_UWORD Para, VS_ULONG ServiceGroupIndex, void *object)
{
	return RustVSScript_GetRawContextType(Para, ServiceGroupIndex, object);
}

static VS_PARAPKGPTR VSScript_RawToParaPkg( VS_UWORD Para, VS_ULONG ServiceGroupIndex,void *Object )
{
	return RustVSScript_RawToParaPkg( Para, ServiceGroupIndex,Object );
}

static VS_BOOL VSScript_AttachRawContext(VS_UWORD Para, VS_ULONG ServiceGroupIndex, void *object, VS_CHAR *ContextName, VS_BOOL IsClass, VS_CHAR *ContextInfo)
{
	return RustVSScript_AttachRawContext(Para,ServiceGroupIndex,object,ContextName,IsClass,ContextInfo);
}

#endif

static VS_BOOL VSScript_InitRaw(VS_UWORD Para, void *BasicSRPInterface, void *SRPInterface)
{
	return RustVSScript_InitRaw(Para,BasicSRPInterface,SRPInterface);
}

static VS_INT32 starrust_starcore_Register(VS_BOOL NeedUnLoad)
{
	struct StructOfVSScriptContext ScriptContext;

	if( StarRust_SRPControlInterface == NULL )
		return -1;
	memset(&ScriptContext,0,sizeof(struct StructOfVSScriptContext));
	ScriptContext.InitRawProc = VSScript_InitRaw;	
#if defined(STAR_RUST_USERAW)	
	ScriptContext.GetRawContextTypeProc = starrust_VSScript_GetRawContextType;
	ScriptContext.AttachRawContextProc = VSScript_AttachRawContext;
	ScriptContext.RawToParaPkgProc = VSScript_RawToParaPkg;
#endif	
    if( NeedUnLoad != VS_TRUE )
		Star_SRPControl_RegScriptInterface(StarRust_SRPControlInterface,"rust", &ScriptContext, 0, NULL); 
	else
		Star_SRPControl_RegScriptInterface(StarRust_SRPControlInterface,"rust", &ScriptContext, 0, starrust_starcore_ScriptTerm); 
    return 0;
}

VS_ULONG starrust_starcore_Load()
{
	VS_CHAR ModuleName[512];
	VS_INT32 Result;
	
	if( StarRust_ModuleInitFlag == VS_TRUE )
	    return 0;	
	if( vs_string_strlen(StarRust_g_CoreLibraryPath) == 0){
		VS_CHAR EnvModuleName[128];
		VS_CHAR CurrentPath[512];
			
		vs_dir_getcwd(CurrentPath,512);
		if( vs_get_env("SRPMODULE",EnvModuleName,128) == VS_FALSE )
			sprintf(EnvModuleName,"libstarcore");
		if( vs_string_strlen(StarRust_g_ShareLibraryPath) == 0){
			sprintf(ModuleName,"%s/%s%s",CurrentPath,EnvModuleName,VS_MODULEEXT);
			hDllInstance = vs_dll_open( ModuleName );
			//if( vs_file_exist(ModuleName) == VS_FALSE )
			if( hDllInstance == NULL ){
			    sprintf(ModuleName,"%s%s%s",VS_COREPATH,EnvModuleName,VS_MODULEEXT);
				hDllInstance = vs_dll_open( ModuleName );
			}
		}else{
			sprintf(ModuleName,"%s/%s%s",StarRust_g_ShareLibraryPath,EnvModuleName,VS_MODULEEXT);
			hDllInstance = vs_dll_open( ModuleName );
		}
		if( hDllInstance == NULL ){
			sprintf(ModuleName,"%s/%s%s",CurrentPath,EnvModuleName,VS_MODULEEXT);
			hDllInstance = vs_dll_open( ModuleName );
			if( hDllInstance == NULL ){
			    sprintf(ModuleName,"%s%s%s",VS_COREPATH,EnvModuleName,VS_MODULEEXT);
				hDllInstance = vs_dll_open( ModuleName );
			}
		}
	}else{
		VS_CHAR EnvModuleName[128];
		VS_CHAR CurrentPath[512];
		
		vs_dir_getcwd(CurrentPath,512);			
		if( vs_get_env("SRPMODULE",EnvModuleName,128) == VS_FALSE )
			sprintf(EnvModuleName,"libstarcore");
		sprintf(ModuleName,"%s/%s%s",StarRust_g_CoreLibraryPath,EnvModuleName,VS_MODULEEXT);
		hDllInstance = vs_dll_open( ModuleName );
		if( hDllInstance == NULL ){
			if( vs_string_strlen(StarRust_g_ShareLibraryPath) == 0){
				sprintf(ModuleName,"%s/%s%s",CurrentPath,EnvModuleName,VS_MODULEEXT);
				hDllInstance = vs_dll_open( ModuleName );
				if( hDllInstance == NULL ){
				    sprintf(ModuleName,"%s%s%s",VS_COREPATH,EnvModuleName,VS_MODULEEXT);
					hDllInstance = vs_dll_open( ModuleName );
				}
			}else{
				sprintf(ModuleName,"%s/%s%s",StarRust_g_ShareLibraryPath,EnvModuleName,VS_MODULEEXT);
				hDllInstance = vs_dll_open( ModuleName );
			}
			if( hDllInstance == NULL ){
				sprintf(ModuleName,"%s/%s%s",CurrentPath,EnvModuleName,VS_MODULEEXT);
				hDllInstance = vs_dll_open( ModuleName );
				if( hDllInstance == NULL ){
					sprintf(ModuleName,"%s%s%s",VS_COREPATH,EnvModuleName,VS_MODULEEXT);
					hDllInstance = vs_dll_open( ModuleName );
				}
			}
		}
	}
	if( hDllInstance != NULL ){		
		ModuleLoadFlag = VS_TRUE;
			
		RegisterCallBackInfoProc = (VSCore_RegisterCallBackInfoProc)vs_dll_sym( hDllInstance, VSCORE_REGISTERCALLBACKINFO_NAME );
		UnRegisterCallBackInfoProc = (VSCore_UnRegisterCallBackInfoProc)vs_dll_sym( hDllInstance, VSCORE_UNREGISTERCALLBACKINFO_NAME );
		VSInitProc = (VSCore_InitProc)vs_dll_sym( hDllInstance, VSCORE_INIT_NAME );
		VSTermProc = (VSCore_TermProc)vs_dll_sym( hDllInstance, VSCORE_TERM_NAME );
		VSTermExProc = (VSCore_TermExProc)vs_dll_sym( hDllInstance, VSCORE_TERMEX_NAME );
		HasInitProc = (VSCore_HasInitProc)vs_dll_sym( hDllInstance, VSCORE_HASINIT_NAME );
		QueryControlInterfaceProc = (VSCore_QueryControlInterfaceProc)vs_dll_sym( hDllInstance, VSCORE_QUERYCONTROLINTERFACE_NAME );						
        Control_GetCFunctionTable = (SRPControl_GetCFunctionTable_Proc)vs_dll_sym(hDllInstance,"SRPControl_GetCFunctionTable");
		
		//--Init
		if( HasInitProc() == VS_FALSE ){
			VS_STARCONFIGEX m_config;

			memset(&m_config,0,sizeof(VS_STARCONFIGEX));
			VS_STRNCPY(m_config.ShareLibraryPath,StarRust_g_ShareLibraryPath,512);
			VS_STRNCPY(m_config.CoreLibraryPath,StarRust_g_CoreLibraryPath,512);
			VS_STRNCPY(m_config.CoreOperationPath,StarRust_g_CoreOperationPath,512);
			Result = VSInitProc( VS_TRUE, VS_TRUE, "",0, "", 0,&m_config );
			if( Result != VSINIT_ERROR ){
				StarRust_SRPControlInterface = QueryControlInterfaceProc();
				g_star_CoreFunctionTbl = (struct StructOfVSStarCoreInterfaceTable *)Control_GetCFunctionTable(StarRust_SRPControlInterface);
				
				starrust_starcore_Register(VS_TRUE);
				StarRust_ScriptInterfaceIndex = g_star_CoreFunctionTbl->SRPControl_GetScriptInterfaceIndex(StarRust_SRPControlInterface,"rust");								
				
				StarRust_SRPCoreShellInterface = g_star_CoreFunctionTbl->SRPControl_GetCoreShellInterface(StarRust_SRPControlInterface);	            
				StarRust_ModuleInitFlag = VS_TRUE;
			}
		}else{
			StarRust_SRPControlInterface = QueryControlInterfaceProc();
			StarRust_ModuleInitFlag = VS_TRUE;
		}
	}else
		return -1;
	return 0;	
}

VS_ULONG starrust_starcore_Init(VS_BOOL ServerFlag, VS_BOOL ShowMenuFlag,VS_BOOL ShowOutWndFlag, VS_BOOL SRPPrintFlag, VS_CHAR *DebugInterface, VS_INT32 DebugPortNumber, VS_CHAR *ClientInterface, VS_INT32 ClientPortNumber)
{
	VS_INT32 Result;
	VS_STARCONFIGEX m_config;

	if( ModuleLoadFlag == VS_FALSE ){
		return 0;
	}
	if( StarRust_ModuleLoadFromStarcore == VS_TRUE ){
		return 0; /*--load from starcore, can not init */
	}
	memset(&m_config,0,sizeof(VS_STARCONFIGEX));
	VS_STRNCPY(m_config.ShareLibraryPath,StarRust_g_ShareLibraryPath,512);
	VS_STRNCPY(m_config.CoreLibraryPath,StarRust_g_CoreLibraryPath,512);
	VS_STRNCPY(m_config.CoreOperationPath,StarRust_g_CoreOperationPath,512);
	Result = VSInitProc( ServerFlag, SRPPrintFlag, DebugInterface,DebugPortNumber, ClientInterface, ClientPortNumber,&m_config );
	if( Result != VSINIT_ERROR )
		StarRust_ModuleInitFlag = VS_TRUE;	
	return 0;
}

/*-----------------------------------------------------------------------------*/
extern void *StarRust_GetFunctionTbl_FromContol(void *SRPControl);

/*SRPDLLEXPORT*/ VS_BOOL SRPAPI star_rust_ScriptInit(VS_CHAR *ScriptName, VS_CHAR *Para, VSCore_RegisterCallBackInfoProc In_RegisterCallBackInfoProc, VSCore_UnRegisterCallBackInfoProc In_UnRegisterCallBackInfoProc, VSCore_InitProc In_InitProc, VSCore_TermProc In_TermProc, VSCore_TermExProc In_TermExProc, VSCore_HasInitProc In_HasInitProc, VSCore_QueryControlInterfaceProc In_QueryControlInterfaceProc, void *VirtualMachine)
{	
	if( vs_string_stricmp(ScriptName,"RUST") != 0 ){
		return VS_FALSE;
	}
	
	StarRust_SRPControlInterface = In_QueryControlInterfaceProc();
	g_star_CoreFunctionTbl = StarRust_GetFunctionTbl_FromContol(StarRust_SRPControlInterface);
	
	/*--check version--*/
    if( g_star_CoreFunctionTbl -> CoreMainVerson < VS_MAINVERSION ){
		Star_SRPControl_Release(StarRust_SRPControlInterface);
		StarRust_SRPControlInterface = NULL;
		return VS_FALSE;
	}else if( g_star_CoreFunctionTbl -> CoreMainVerson == VS_MAINVERSION ){
		if( g_star_CoreFunctionTbl -> CoreSubVerson < VS_SUBVERSION ){
			Star_SRPControl_Release(StarRust_SRPControlInterface);
			StarRust_SRPControlInterface = NULL;			
 			return VS_FALSE;
		}else if( g_star_CoreFunctionTbl -> CoreSubVerson == VS_SUBVERSION ){
			if( g_star_CoreFunctionTbl -> CoreBuildVerson < VS_BUILDVERSION ){
				Star_SRPControl_Release(StarRust_SRPControlInterface);
				StarRust_SRPControlInterface = NULL;				
	 			return VS_FALSE	;		
			}
		}
	}
	
	RegisterCallBackInfoProc = In_RegisterCallBackInfoProc;
	UnRegisterCallBackInfoProc = In_UnRegisterCallBackInfoProc;
	VSInitProc = In_InitProc;
	VSTermProc = In_TermProc;
	VSTermExProc = In_TermExProc;
	HasInitProc = In_HasInitProc;
	QueryControlInterfaceProc = In_QueryControlInterfaceProc;	
					
	starrust_starcore_Register(VS_TRUE);
	StarRust_ScriptInterfaceIndex = g_star_CoreFunctionTbl->SRPControl_GetScriptInterfaceIndex(StarRust_SRPControlInterface,"rust");								
				
	StarRust_SRPCoreShellInterface = g_star_CoreFunctionTbl->SRPControl_GetCoreShellInterface(StarRust_SRPControlInterface);	            

	StarRust_ModuleInitFlag = VS_TRUE;
	ModuleLoadFlag = VS_TRUE;
	StarRust_ModuleLoadFromStarcore = VS_TRUE;
	
	RustInitFromStarCore();
	
	return VS_TRUE;
}		

/*SRPDLLEXPORT*/ VS_BOOL SRPAPI star_rust_ScriptInit2(VS_CHAR *ScriptName, VS_CHAR *Para, VSCore_RegisterCallBackInfoProc In_RegisterCallBackInfoProc, VSCore_UnRegisterCallBackInfoProc In_UnRegisterCallBackInfoProc, VSCore_InitProc In_InitProc, VSCore_TermProc In_TermProc, VSCore_TermExProc In_TermExProc, VSCore_HasInitProc In_HasInitProc, VSCore_QueryControlInterfaceProc In_QueryControlInterfaceProc, struct StructOfVSStarCoreInterfaceTable *InterfaceTable, void *VirtualMachine)
{		
	StarRust_SRPControlInterface = In_QueryControlInterfaceProc();
	g_star_CoreFunctionTbl = InterfaceTable;	
	
	/*--check version--*/
    if( g_star_CoreFunctionTbl -> CoreMainVerson < VS_MAINVERSION ){
		Star_SRPControl_Release(StarRust_SRPControlInterface);
		StarRust_SRPControlInterface = NULL;
		return VS_FALSE;
	}else if( g_star_CoreFunctionTbl -> CoreMainVerson == VS_MAINVERSION ){
		if( g_star_CoreFunctionTbl -> CoreSubVerson < VS_SUBVERSION ){
			Star_SRPControl_Release(StarRust_SRPControlInterface);
			StarRust_SRPControlInterface = NULL;			
 			return VS_FALSE;
		}else if( g_star_CoreFunctionTbl -> CoreSubVerson == VS_SUBVERSION ){
			if( g_star_CoreFunctionTbl -> CoreBuildVerson < VS_BUILDVERSION ){
				Star_SRPControl_Release(StarRust_SRPControlInterface);
				StarRust_SRPControlInterface = NULL;				
	 			return VS_FALSE	;		
			}
		}
	}
	
	RegisterCallBackInfoProc = In_RegisterCallBackInfoProc;
	UnRegisterCallBackInfoProc = In_UnRegisterCallBackInfoProc;
	VSInitProc = In_InitProc;
	VSTermProc = In_TermProc;
	VSTermExProc = In_TermExProc;
	HasInitProc = In_HasInitProc;
	QueryControlInterfaceProc = In_QueryControlInterfaceProc;	

	StarRust_ModuleInitFlag = VS_TRUE;
	ModuleLoadFlag = VS_TRUE;
	StarRust_ModuleLoadFromStarcore = VS_TRUE;	
					
	starrust_starcore_Register(VS_TRUE);
	StarRust_ScriptInterfaceIndex = g_star_CoreFunctionTbl->SRPControl_GetScriptInterfaceIndex(StarRust_SRPControlInterface,"rust");								
				
	StarRust_SRPCoreShellInterface = g_star_CoreFunctionTbl->SRPControl_GetCoreShellInterface(StarRust_SRPControlInterface);	            
	
	RustInitFromStarCore();
	
	return VS_TRUE;
}		

/*SRPDLLEXPORT*/ VS_BOOL SRPAPI starrust_StarCoreService_Init2(void *StarCore, struct StructOfVSStarCoreInterfaceTable *InterfaceTable )
{
	if( StarCore == NULL )
	    return VS_FALSE;
	g_star_CoreFunctionTbl = InterfaceTable;
	StarRust_SRPControlInterface = Star_StarCore_GetControlInterface(StarCore);
	Star_SRPControl_AddRef(StarRust_SRPControlInterface);
		
	/*--check version--*/
    if( g_star_CoreFunctionTbl -> CoreMainVerson < VS_MAINVERSION ){
		Star_SRPControl_Release(StarRust_SRPControlInterface);
		StarRust_SRPControlInterface = NULL;
		return VS_FALSE;
	}else if( g_star_CoreFunctionTbl -> CoreMainVerson == VS_MAINVERSION ){
		if( g_star_CoreFunctionTbl -> CoreSubVerson < VS_SUBVERSION ){
			Star_SRPControl_Release(StarRust_SRPControlInterface);
			StarRust_SRPControlInterface = NULL;			
 			return VS_FALSE;
		}else if( g_star_CoreFunctionTbl -> CoreSubVerson == VS_SUBVERSION ){
			if( g_star_CoreFunctionTbl -> CoreBuildVerson < VS_BUILDVERSION ){
				Star_SRPControl_Release(StarRust_SRPControlInterface);
				StarRust_SRPControlInterface = NULL;				
	 			return VS_FALSE	;		
			}
		}
	}
	/*starrust_starcore_Register(VS_FALSE);  not register interface*/
	g_star_CoreFunctionTbl->SRPControl_SetScriptInterfaceIndex(StarRust_SRPControlInterface,"rust");
	StarRust_ScriptInterfaceIndex = g_star_CoreFunctionTbl->SRPControl_GetScriptInterfaceIndex(StarRust_SRPControlInterface,"rust");								
				
	StarRust_SRPCoreShellInterface = g_star_CoreFunctionTbl->SRPControl_GetCoreShellInterface(StarRust_SRPControlInterface);	            
	StarRust_ModuleInitFlag = VS_TRUE;
	ModuleLoadFlag = VS_TRUE;
	StarRust_ModuleLoadFromStarcore = VS_TRUE;
	
	RustInitFromStarCore();
	
	void *BasicSRPInterface = Star_StarCore_GetBasicInterface(StarCore);	
	void *SRPInterface = Star_SRPBasic_GetSRPInterface(BasicSRPInterface,Star_SRPBasic_QueryActiveService(BasicSRPInterface,NULL),"","");	
	if( SRPInterface != NULL ){
	    RustVSScript_InitRaw(0,BasicSRPInterface,SRPInterface);
		Star_SRPI_Release(SRPInterface);
	}

	return VS_TRUE; 
}

/*SRPDLLEXPORT*/ void SRPAPI starrust_StarCoreService_Term2(void *StarCore, struct StructOfVSStarCoreInterfaceTable *InterfaceTable )
{
	starrust_starcore_ScriptTerm();
}

VS_INT64 starrust_CoreHandle()
{
	return (VS_INT64)(VS_UWORD)hDllInstance;
}

VS_UWORD starrust_MsgCallBack( VS_ULONG ServiceGroupID, VS_ULONG uMsg, VS_UWORD wParam, VS_UWORD lParam, VS_BOOL *IsProcessed, VS_UWORD Para )
{
	return RustMsgCallBack(ServiceGroupID,uMsg,wParam,lParam,IsProcessed,Para );
}

void starrust_RegisterCallBackInfo( void* MsgCallBackProc, VS_UWORD MsgCallBackPara )
{
	if( RegisterCallBackInfoProc == NULL )
	    return;
	RegisterCallBackInfoProc((VS_MsgCallBackProc)MsgCallBackProc,MsgCallBackPara);
}
void starrust_UnRegisterCallBackInfo( void* MsgCallBackProc, VS_UWORD MsgCallBackPara )
{
	if( UnRegisterCallBackInfoProc == NULL )
	    return;
	UnRegisterCallBackInfoProc((VS_MsgCallBackProc)MsgCallBackProc,MsgCallBackPara);
}

void starrust_SRPDispatchRequestCallBack(VS_UWORD Para)
{
	RustSRPDispatchRequestCallBack();
}
 
void starrust_Term( )
{
	if( VSTermProc != NULL )
	    VSTermProc();
}

void starrust_TermEx( )
{
	if( VSTermExProc != NULL )
	    VSTermExProc();
}

void starrust_inituuid(VS_UUID *u)
{
	INIT_UUID( (*u) );
}

void *starrust_MovePointer(void *In,VS_INT32 Step)
{
	return (void *)(((VS_INT8 *)In) + Step);
}

VS_CHAR *starrust_GetCharByIndex(VS_CHAR *Ptr,VS_INT32 Index)
{
	return Ptr + Index;
}

void starrust_starcore_unLoad()
{
	if( StarRust_ModuleLoadFromStarcore == VS_TRUE )
	    return;
	if( VSTermExProc != NULL )
		VSTermExProc();
	if( hDllInstance != NULL )
		vs_dll_close(hDllInstance);
	hDllInstance = NULL;
	StarRust_ModuleInitFlag = VS_FALSE;	
}

VS_BOOL starrust_uuidisequal(VS_UUID *u1,VS_UUID *u2)
{
	return UUID_ISEQUAL((*u1),(*u2));
}

VS_BOOL starrust_uuidisvalid(VS_UUID *u)
{
	if( UUID_ISINVALID( (*u) ) )
	    return VS_TRUE;
	else
	    return VS_FALSE;
}

void *starrust_ToPointer64(VS_INT64 in)
{
	return (void *)(VS_UWORD)in;
}

void *starrust_ToPointer(VS_UWORD in)
{
	return (void *)in;
}

void *starrust_GetUWordPointer(VS_UWORD in)
{
	return ((void **)in)[0];
}
void starrust_SetUWordPointer(VS_UWORD in,void *ptr)
{
	((void **)in)[0] = ptr;
}

VS_INT64 starrust_PointerToInt64(void *Ptr)
{
	return (VS_INT64)((VS_UINT64)((VS_UWORD)Ptr));
}

VS_UWORD starrust_PointerToUWord(void *Ptr)
{
	return (VS_UWORD)Ptr;
}

VS_CHAR *starrust_IPToString(SOCKADDR_IN *ip)
{
	static VS_CHAR Buf[256];
    sprintf(Buf,"%d.%d.%d.%d",((struct _in_addr *)&ip->sin_addr)->S_un.S_un_b.s_b1,
			((struct _in_addr *)&ip->sin_addr)->S_un.S_un_b.s_b2,
			((struct _in_addr *)&ip->sin_addr)->S_un.S_un_b.s_b3,
			((struct _in_addr *)&ip->sin_addr)->S_un.S_un_b.s_b4);	
	return Buf;
}

struct StructOfObjectRefInSrvGroup *starrust_MallocStructOfObjectRefInSrvGroup()
{
	return (struct StructOfObjectRefInSrvGroup *)malloc(sizeof(struct StructOfObjectRefInSrvGroup));
}

void starrust_FreeStructOfObjectRefInSrvGroup(struct StructOfObjectRefInSrvGroup *Val)
{
	if( Val == NULL )
	    return;
	free(Val);
}

#if defined(STAR_RUST_USERAW)
VS_INT32 starrust_SizeOfStructOfRustRawContext()
{
	return sizeof(struct StructOfRustRawContext);
}

struct classRawContextRefItem *starrust_MallocclassRawContextRefItem()
{
	return (struct classRawContextRefItem *)malloc(sizeof(struct classRawContextRefItem));
}

struct StructOfRustRawContext_Para *starrust_MallocStructOfRustRawContext_Para()
{
	return (struct StructOfRustRawContext_Para *)malloc(sizeof(struct StructOfRustRawContext_Para));
}
#endif

void starrust_SRPI_ProcessError(void *c_this,VS_INT32 AlarmLevel,const VS_CHAR *SourceName,VS_INT32 LineIndex,const VS_CHAR *Info)
{
	g_star_CoreFunctionTbl->SRPI_ProcessError(c_this,AlarmLevel,SourceName,LineIndex,"%s",Info);
}

void starrust_SRPBasic_ProcessError(void *c_this,VS_INT32 AlarmLevel,const VS_CHAR *SourceName,VS_INT32 LineIndex,const VS_CHAR *Info)
{
	g_star_CoreFunctionTbl->SRPBasic_ProcessError(c_this,AlarmLevel,SourceName,LineIndex,"%s",Info);
}

void starrust_SRPControl_ProcessError(void *c_this,VS_INT32 AlarmLevel,const VS_CHAR *SourceName,VS_INT32 LineIndex,const VS_CHAR *Info)
{
	g_star_CoreFunctionTbl->SRPControl_ProcessError(c_this,AlarmLevel,SourceName,LineIndex,Info);
}

VS_UWORD starrust_SRPI_ScriptCall(void *c_this,void *Object,VS_ULONG *RetCode,const VS_CHAR *FunctionName,const VS_CHAR *TypeSet)
{
	return g_star_CoreFunctionTbl->SRPI_ScriptCall(c_this,Object,RetCode,FunctionName,TypeSet);
}

void starrust_FreeScriptObject(VS_UWORD Para, VS_UWORD ScriptObject)
{	
    RustFreeScriptObject(ScriptObject);
}

void starrust_ObjectIDChangeNotify(void *Object,VS_UWORD Para,VS_UUID *NewObjectID)
{
	RustObjectIDChangeNotify(Object,Para, NewObjectID);
}

void starrust_ObjectFreeNotify(void *Object,VS_UWORD Para)
{
	RustObjectFreeNotify(Object,Para);
}


VS_INT32 starrust_SRPObject_ScriptCallBack( void *L )
{
	return RustSRPObject_ScriptCallBack(L);
}

#if defined(STAR_RUST_USERAW)

VS_INT32 starrust_VSScript_RustRawContext_GeneralFunction(void *L)
{
	return RustRawContext_GeneralFunction(L);
}

VS_BOOL starrust_VSScript_RustRawContext_LuaFuncFilter(void *object, void *ForWhichObject, VS_CHAR *FuncName, VS_UWORD Para)
{
	return RustRawContext_LuaFuncFilter(object,ForWhichObject,FuncName,Para);
}

VS_BOOL starrust_VSScript_RustRawContext_RegGetValue(void *Object, void *ForWhichObject, VS_CHAR *Name, VS_UWORD Para, VS_BOOL GetAllRawAttributeFlag)
{
	return RustRawContext_RegGetValue(Object,ForWhichObject,Name,Para,GetAllRawAttributeFlag);
}

VS_BOOL starrust_VSScript_RustRawContext_RegSetValue(void *Object, void *ForWhichObject, VS_CHAR *Name, VS_INT32 Index, VS_UWORD Para)
{
	return RustRawContext_RegSetValue(Object,ForWhichObject,Name,Index,Para);
}

void starrust_VSScript_RustRawContext_NewFunctionCallBack(void *Object, void *DesObject, VS_CHAR *FuncName, VS_UWORD Para)
{
	return;
}
#endif

void *starrust_mutex_init( )
{
	VS_MUTEX *mutex = (VS_MUTEX *)malloc(sizeof(VS_MUTEX));
	vs_mutex_init(mutex);
	return (void *)mutex;
}

VS_BOOL starrust_mutex_lock( void *mutex)
{
	return vs_mutex_lock((VS_MUTEX *)mutex);
}
void starrust_mutex_unlock( void *mutex)
{
	vs_mutex_unlock((VS_MUTEX *)mutex);
}
void starrust_mutex_destory( void *mutex)
{
	vs_mutex_destory((VS_MUTEX *)mutex);
}

VS_CHAR *vs_string_strcpy(VS_CHAR* dest, const VS_CHAR *src)
{
	return strcpy(dest,src);
}

int starrust_fclose( FILE *fp )
{
	return fclose(fp);
}

size_t starrust_fwrite(const void* buffer, size_t size, size_t count, FILE* stream)
{
    return fwrite(buffer,size,count,stream);
}

size_t starrust_fread(void* buffer, size_t size, size_t count, FILE* stream)
{
    return fread(buffer,size,count,stream);
}

VS_INT32 starrust_fseek(FILE *stream, VS_INT32 offset, VS_INT32 fromwhere)
{
	return (VS_INT32)fseek(stream,(long)offset,(int)fromwhere);
}

VS_INT32 starrust_ftell(FILE *stream)
{
	return (VS_INT32)ftell(stream);
}

char *starrust_strncpy ( char *destination, const char *source, size_t num )
{
	if( num == 0 )
	    return destination;
    char *res = strncpy ( destination,source,num );
	*(destination + num - 1 ) = 0;
	return res;
}