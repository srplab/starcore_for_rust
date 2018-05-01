
extern {
    fn starrust_MallocStructOfObjectRefInSrvGroup() -> *mut StructOfObjectRefInSrvGroup;
    fn starrust_FreeStructOfObjectRefInSrvGroup(Val: *mut StructOfObjectRefInSrvGroup);

    static StarRust_SRPControlInterface: *mut c_void; 
    static StarRust_SRPCoreShellInterface: *mut c_void;
    static StarRust_ScriptInterfaceIndex: i32;

    static StarRust_ModuleInitFlag: VS_BOOL;
    static StarRust_ModuleLoadFromStarcore: VS_BOOL;

    fn starrust_starcore_Load() -> u32;
    fn starrust_starcore_unLoad();
    fn starrust_CoreHandle() -> i64;

    fn starrust_starcore_Init(ServerFlag: VS_BOOL,ShowMenuFlag: VS_BOOL,ShowOutWndFlag: VS_BOOL,SRPPrintFlag: VS_BOOL,DebugInterface: *const c_char,DebugPortNumber: i32,ClientInterface: *const c_char, ClientPortNumber: i32) -> u32;
    fn starrust_inituuid(u: *mut VS_UUID);
    fn starrust_uuidisequal(u1: *const VS_UUID,u2: *const VS_UUID) -> VS_BOOL;
    fn starrust_uuidisvalid(u: *const VS_UUID) -> VS_BOOL;

    fn starrust_MovePointer(In: *const c_void, Step: i32) -> *const c_void;

    fn starrust_MsgCallBack(ServiceGroupID:u32,uMsg:u32,wParam:usize,lParam:usize,IsProcessed: *mut VS_BOOL,Para:usize) -> usize;
    fn starrust_RegisterCallBackInfo(MsgCallBackProc: *const c_void,MsgCallBackPara:usize);
    fn starrust_UnRegisterCallBackInfo(MsgCallBackProc: *const c_void,MsgCallBackPara:usize);

    fn starrust_SRPDispatchRequestCallBack(Para:usize);

    fn starrust_Term();
    fn starrust_TermEx();

    fn starrust_ToPointer64(inval: i64) -> *mut c_void;
    fn starrust_ToPointer(inval: usize) -> *mut c_void;
    fn starrust_GetUWordPointer(inval: usize) -> *mut c_void; /*--get pointer saved in VS_UWORD--*/
    fn starrust_SetUWordPointer(inval: usize,ptr: *const c_void); /*--set pointer to be saved in VS_UWORD--*/

    fn starrust_IPToString(ip: *const VSSOCKADDR_IN) -> *mut c_char;
    fn starrust_PointerToInt64(Ptr: *const c_void) -> i64;
    fn starrust_PointerToUWord(Ptr: *const c_void) -> usize;
    fn starrust_GetCharByIndex(Ptr: *const c_char,Index:i32) -> *mut c_char;

    fn starrust_SRPI_ProcessError(c_this: *const c_void,AlarmLevel:i32,SourceName:*const c_char,LineIndex:i32,Info:*const c_char);
    fn starrust_SRPBasic_ProcessError(c_this: *const c_void,AlarmLevel:i32,SourceName:*const c_char,LineIndex:i32,Info:*const c_char);
    fn starrust_SRPControl_ProcessError(c_this: *const c_void,AlarmLevel:i32,SourceName:*const c_char,LineIndex:i32,Info:*const c_char);

    /*---only support no parameter--*/
    fn starrust_SRPI_ScriptCall(c_this: *const c_void,Object: *const c_void,RetCode:*mut u32,FunctionName:*const c_char,TypeSet:*const c_char) -> usize;

    fn starrust_FreeScriptObject(Para:usize,ScriptObject:usize);
    fn starrust_ObjectIDChangeNotify(Object: *const c_void,Para:usize,NewObjectID: *const VS_UUID);
    fn starrust_ObjectFreeNotify(Object: *const c_void,Para:usize);
    fn starrust_SRPObject_ScriptCallBack(L: *const c_void) -> i32;

    fn starrust_mutex_init( ) -> *mut c_void;
    fn starrust_mutex_lock( mutex: *const c_void) -> VS_BOOL;
    fn starrust_mutex_unlock( mutex: *const c_void);
    fn starrust_mutex_destory( mutex: *const c_void);

    fn vs_string_strlen(val: *const c_char) -> usize;
    fn vs_string_strcmp(string1: *const c_char,string2: *const c_char) -> i32;
    fn vs_string_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    fn vs_dir_getcwd(Buf:*mut c_char,Size:u32) -> VS_BOOL;

    fn vs_get_env(Name: *const c_char,Buf: *mut c_char,BufSize: i32) -> VS_BOOL;
    fn vs_set_env(Name: *const c_char,Buf: *const c_char) -> VS_BOOL;

    fn vs_memset(DesBuf: *mut c_void,c:i8,Size:isize);
    fn vs_memcpy(DesBuf: *mut c_void,SrcBuf: *const c_void,Size:isize);

    fn vs_file_fopen(FileName: *const c_char,mode: *const c_char ) -> *const c_void;
    fn starrust_fread ( buffer: *mut c_void, size:usize, count:usize, stream:*const c_void) -> usize;
    fn starrust_fwrite(buffer: *const c_void, size:usize, count:usize, stream:*const c_void) -> usize;
    fn starrust_fclose(stream:*const c_void) -> i32;
    fn starrust_fseek(stream:*const c_void, offset:i32, fromwhere: i32) -> i32;
    fn starrust_ftell(stream:*const c_void) -> i32;

    fn starrust_strncpy(destination: *mut c_char, source: *const c_char, num:isize) -> *mut c_char;

    /*--export functions--*/
    fn starrust_StarCoreService_Init2(StarCore:*const c_void, InterfaceTable:*const c_void ) -> VS_BOOL;
    fn starrust_StarCoreService_Term2(StarCore:*const c_void, InterfaceTable:*const c_void );
}