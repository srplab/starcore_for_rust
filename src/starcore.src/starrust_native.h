#ifndef _STAR_RUST_H
#define _STAR_RUST_H 1

//#define STAR_RUST_USERAW

struct StructOfObjectRefInSrvGroup{
	VS_UWORD RefItem;
	VS_BOOL IncreaseRef;  /*if == VS_FALSE, then use weak global ref, otherwise, use global ref */
};
struct StructOfObjectRefInSrvGroup *starrust_MallocStructOfObjectRefInSrvGroup();
void starrust_FreeStructOfObjectRefInSrvGroup(struct StructOfObjectRefInSrvGroup *Val);

#if defined(STAR_RUST_USERAW)
struct classRawContextRefItem{
	VS_UWORD o;
	VS_UUID  ObjectID;
	VS_BOOL IsClass;	
	struct classRawContextRefItem *Up;
	struct classRawContextRefItem *Down;
};

struct StructOfRustRawContext_Para{
	VS_ULONG ServiceGroupIndex;
	void *object;
};


struct StructOfRustRawContext{
	struct StructOfCommonRawContextHeader ContextHeader;
	struct classRawContextRefItem *RefItem;
	struct StructOfRustRawContext_Para *Para;	
};

VS_INT32 starrust_SizeOfStructOfRustRawContext();
struct classRawContextRefItem *starrust_MallocclassRawContextRefItem();
struct StructOfRustRawContext_Para *starrust_MallocStructOfRustRawContext_Para();
#endif

extern void *StarRust_SRPControlInterface;
extern void *StarRust_SRPCoreShellInterface;
extern VS_INT32 StarRust_ScriptInterfaceIndex;

extern VS_BOOL StarRust_ModuleInitFlag;
extern VS_BOOL StarRust_ModuleLoadFromStarcore;

extern VS_CHAR  StarRust_g_ShareLibraryPath[];
extern VS_CHAR  StarRust_g_CoreLibraryPath[];
extern VS_CHAR  StarRust_g_CoreOperationPath[];

VS_ULONG starrust_starcore_Load();
void starrust_starcore_unLoad();
VS_INT64 starrust_CoreHandle();

VS_ULONG starrust_starcore_Init(VS_BOOL ServerFlag, VS_BOOL ShowMenuFlag,VS_BOOL ShowOutWndFlag, VS_BOOL SRPPrintFlag, VS_CHAR *DebugInterface, VS_INT32 DebugPortNumber, VS_CHAR *ClientInterface, VS_INT32 ClientPortNumber);

void starrust_inituuid(VS_UUID *u);
VS_BOOL starrust_uuidisequal(VS_UUID *u1,VS_UUID *u2);
VS_BOOL starrust_uuidisvalid(VS_UUID *u);

void *starrust_MovePointer(void *In,VS_INT32 Step);

VS_UWORD starrust_MsgCallBack( VS_ULONG ServiceGroupID, VS_ULONG uMsg, VS_UWORD wParam, VS_UWORD lParam, VS_BOOL *IsProcessed, VS_UWORD Para );
void starrust_RegisterCallBackInfo( void* MsgCallBackProc, VS_UWORD MsgCallBackPara );
void starrust_UnRegisterCallBackInfo( void* MsgCallBackProc, VS_UWORD MsgCallBackPara );

void starrust_SRPDispatchRequestCallBack(VS_UWORD Para);

void starrust_Term( );
void starrust_TermEx( );

void *starrust_ToPointer64(VS_INT64 in);
void *starrust_ToPointer(VS_UWORD in);
void *starrust_GetUWordPointer(VS_UWORD in);  /*--get pointer saved in VS_UWORD--*/
void starrust_SetUWordPointer(VS_UWORD in,void *ptr);  /*--set pointer to be saved in VS_UWORD--*/
VS_CHAR *starrust_IPToString(SOCKADDR_IN *ip);
VS_INT64 starrust_PointerToInt64(void *Ptr);
VS_UWORD starrust_PointerToUWord(void *Ptr);
VS_CHAR *starrust_GetCharByIndex(VS_CHAR *Ptr,VS_INT32 Index);

void starrust_SRPI_ProcessError(void *c_this,VS_INT32 AlarmLevel,const VS_CHAR *SourceName,VS_INT32 LineIndex,const VS_CHAR *Info);
void starrust_SRPBasic_ProcessError(void *c_this,VS_INT32 AlarmLevel,const VS_CHAR *SourceName,VS_INT32 LineIndex,const VS_CHAR *Info);
void starrust_SRPControl_ProcessError(void *c_this,VS_INT32 AlarmLevel,const VS_CHAR *SourceName,VS_INT32 LineIndex,const VS_CHAR *Info);

/*---only support no parameter--*/
VS_UWORD starrust_SRPI_ScriptCall(void *c_this,void *Object,VS_ULONG *RetCode,const VS_CHAR *FunctionName,const VS_CHAR *TypeSet);


void starrust_FreeScriptObject(VS_UWORD Para, VS_UWORD ScriptObject);
void starrust_ObjectIDChangeNotify(void *Object,VS_UWORD Para,VS_UUID *NewObjectID);
void starrust_ObjectFreeNotify(void *Object,VS_UWORD Para);
VS_INT32 starrust_SRPObject_ScriptCallBack( void *L );

#if defined(STAR_RUST_USERAW)
//--not complete
VS_INT32 starrust_VSScript_RustRawContext_GeneralFunction(void *L);
VS_BOOL starrust_VSScript_RustRawContext_LuaFuncFilter(void *object, void *ForWhichObject, VS_CHAR *FuncName, VS_UWORD Para);
VS_BOOL starrust_VSScript_RustRawContext_RegGetValue(void *Object, void *ForWhichObject, VS_CHAR *Name, VS_UWORD Para, VS_BOOL GetAllRawAttributeFlag);
VS_BOOL starrust_VSScript_RustRawContext_RegSetValue(void *Object, void *ForWhichObject, VS_CHAR *Name, VS_INT32 Index, VS_UWORD Para);
void starrust_VSScript_RustRawContext_NewFunctionCallBack(void *Object, void *DesObject, VS_CHAR *FuncName, VS_UWORD Para);
#endif

void *starrust_mutex_init( );
VS_BOOL starrust_mutex_lock( void *mutex);
void starrust_mutex_unlock( void *mutex);
void starrust_mutex_destory( void *mutex);

VS_CHAR *vs_string_strcpy(VS_CHAR* dest, const VS_CHAR *src);

#endif /* !_STAR_RUST_H */

