/*
vsopenapi_c stub file for rust
create by srplab            
create date: 2018-4-13
*/

/*
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
*/

/*
enum VS_QUERYRECORD{}
enum VSSOCKADDR_IN{}
enum StructOfSRPComm_HttpOnRequest{}
enum VSPublicServiceDef{}
enum StructOfVSScriptContext{}
enum StructOfVSServerUrlInfo{}
enum VS_EVENTPARAM_RUNPARAM{}
enum VS_STATISTICINFO{}
enum VS_UPDOWNFILEINFO{}
enum VSWINDOWSTYLE{}
enum VSWINDOWPOSITION{}
enum StructOfVSRunEnv{}
enum VS_TIME{}
enum VS_SERVICEINFO{}
enum VS_VSTRING{}
enum VS_ATTRIBUTEINFO{}
enum VS_COMBOBOXITEM{}
enum VS_FUNCTIONINFO{}
enum VS_OUTEVENTINFO{}
enum VS_EVENTPARAM{}
enum VS_ACTIVESETITEM{}
enum VS_CLIENTINFO{}
enum VS_CLIENTQOS{}
enum VS_UPDOWNFILEMSG{}
enum VS_RECT{}
enum VS_FONT{}
enum VSLuaL_Reg{}
enum VSImportServiceDef{}
enum VS_UUID{}
enum VSWindowlessSiteCallBackInfo{}
*/


#[link(name="vsopenapi_c_stub", kind = "static")]
extern {
    fn Star_SRPComm_Release(c_this: *const c_void);
    fn Star_SRPComm_CreateMsgQueue(c_this: *const c_void, ControlMsgSize: i32, DataMsgSize: i32) -> *const c_void;
    fn Star_SRPComm_DeleteMsgQueue(c_this: *const c_void, MsgHandle: *const c_void);
    fn Star_SRPComm_GetControlMsgBuf(c_this: *const c_void, MsgHandle: *const c_void) -> *mut i8;
    fn Star_SRPComm_GetDataMsgBuf(c_this: *const c_void, MsgHandle: *const c_void) -> *mut i8;
    fn Star_SRPComm_FreeMsgBuf(c_this: *const c_void, MsgHandle: *const c_void, MsgBuf: *mut i8);
    fn Star_SRPComm_AddMsgToQueue(c_this: *const c_void, MsgHandle: *const c_void, MsgBuf: *mut i8) -> i32;
    fn Star_SRPComm_GetMsgFromQueue(c_this: *const c_void, MsgHandle: *const c_void, WaitMark: i32) -> *mut i8;
    fn Star_SRPComm_GetKernelMsgQueue(c_this: *const c_void) -> *const c_void;
    fn Star_SRPComm_AllocKernelMsgClass(c_this: *const c_void) -> u16;
    fn Star_SRPComm_FreeKernelMsgClass(c_this: *const c_void, MsgClass: u16);
    fn Star_SRPComm_RegKernelMsgProc(c_this: *const c_void, MsgClass: u16, KernelMsgProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPComm_UnRegKernelMsgProc(c_this: *const c_void, MsgClass: u16, KernelMsgProc: *const c_void, Para: usize);
    fn Star_SRPComm_RegWebServerMsgProc(c_this: *const c_void, WebServerMsgProc: *const c_void, Para: usize, RunInKernel: VS_BOOL, BufSize: u32) -> VS_BOOL;
    fn Star_SRPComm_UnRegWebServerMsgProc(c_this: *const c_void, WebServerMsgProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPComm_WebServerRelease(c_this: *const c_void, ConnectionID: u32);
    fn Star_SRPComm_TCPSetupServer(c_this: *const c_void, MsgHandle: *const c_void, BufferPkgNum: i32, LocalServerName: *const i8, PortNumber: u16, ConnectionGroup: u32, MsgClass: u16, SocketAddr: *mut VSSOCKADDR_IN) -> u32;
    fn Star_SRPComm_TCPSetupClient(c_this: *const c_void, MsgHandle: *const c_void, BufferPkgNum: i32, ServerName: *const i8, PortNumber: u16, ConnectionGroup: u32, MsgClass: u16) -> u32;
    fn Star_SRPComm_TCPSend(c_this: *const c_void, ConnectionID: u32, Length: i32, FrameBuf: *const i8, MoreData: VS_BOOL) -> i32;
    fn Star_SRPComm_TCPRecv(c_this: *const c_void, ConnectionID: u32, Length: i32, FrameBuf: *mut i8) -> i32;
    fn Star_SRPComm_TCPRecvLine(c_this: *const c_void, SocketIDOfClient: u32, Length: i32, FrameBuf: *mut c_char, ModeDataSize: *mut i32) -> i32;
    fn Star_SRPComm_TCPPreview(c_this: *const c_void, SocketIDOfClient: u32, Length: i32, FrameBuf: *mut i8) -> i32;
    fn Star_SRPComm_TCPDisacrd(c_this: *const c_void, SocketIDOfClient: u32, Length: i32) -> i32;
    fn Star_SRPComm_TCPRelease(c_this: *const c_void, ConnectionID: u32);
    fn Star_SRPComm_UDPSetupServer(c_this: *const c_void, MsgHandle: *const c_void, BufferPkgNum: i32, LocalServerName: *const i8, PortNumber: u16, ConnectionGroup: u32, MsgClass: u16, SocketAddr: *mut VSSOCKADDR_IN) -> u32;
    fn Star_SRPComm_UDPSetupClient(c_this: *const c_void, MsgHandle: *const c_void, BufferPkgNum: i32, ConnectionGroup: u32, MsgClass: u16) -> u32;
    fn Star_SRPComm_UDPSend(c_this: *const c_void, ConnectionID: u32, Length: i32, FrameBuf: *const i8, SockAddr: *mut VSSOCKADDR_IN) -> i32;
    fn Star_SRPComm_UDPRecv(c_this: *const c_void, ConnectionID: u32, Length: *mut i32, FrameBuf: *mut i8, SockAddr: *mut VSSOCKADDR_IN) -> VS_BOOL;
    fn Star_SRPComm_UDPRelease(c_this: *const c_void, ConnectionID: u32);
    fn Star_SRPComm_UDPSetSockAddr(c_this: *const c_void, Name: *const c_char, Port: u16, SockAddr: *mut VSSOCKADDR_IN) -> VS_BOOL;
    fn Star_SRPComm_HttpDownLoad(c_this: *const c_void, MsgHandle: *const c_void, Url: *const c_char, FileName: *const c_char, ConnectionGroup: u32, MsgClass: u16) -> u32;
    fn Star_SRPComm_HttpUpLoad(c_this: *const c_void, MsgHandle: *const c_void, Url: *const c_char, FileName: *const c_char, FileSize: u64, ContentType: *const c_char, MultiPartFlag: VS_BOOL, SaveFileName: *const c_char, ConnectionGroup: u32, MsgClass: u16) -> u32;
    fn Star_SRPComm_HttpDownLoadEx(c_this: *const c_void, MsgHandle: *const c_void, Url: *const c_char, FileName: *const c_char, RequestHeader: *const c_char, ConnectionGroup: u32, MsgClass: u16) -> u32;
    fn Star_SRPComm_HttpUpLoadEx(c_this: *const c_void, MsgHandle: *const c_void, Url: *const c_char, FileName: *const c_char, FileSize: u64, RequestHeader: *const c_char, ConnectionGroup: u32, MsgClass: u16) -> u32;
    fn Star_SRPComm_HttpSend(c_this: *const c_void, ConnectionID: u32, Length: i32, FrameBuf: *const i8, MoreData: VS_BOOL) -> i32;
    fn Star_SRPComm_HttpRecv(c_this: *const c_void, ConnectionID: u32, Length: i32, FrameBuf: *mut i8) -> i32;
    fn Star_SRPComm_HttpRelease(c_this: *const c_void, ConnectionID: u32);
    fn Star_SRPComm_HttpServer(c_this: *const c_void, MsgHandle: *const c_void, LocalServerName: *const i8, PortNumber: u16, ConnectionGroup: u32, MsgClass: u16, SocketAddr: *mut VSSOCKADDR_IN, MaxPostSize: u32) -> u32;
    fn Star_SRPComm_FormatRspHeader(c_this: *const c_void, RspInfo: *const c_char, ServerInfo: *const c_char, Connection: *const c_char, ContentType: *const c_char, ContentLength: u64, Buf: *mut c_char);
    fn Star_SRPComm_ParsePara(c_this: *const c_void, Info: *const c_char, ParaName: *const c_char, ParaBuf: *mut c_char, ParaBufSize: i32) -> VS_BOOL;
    fn Star_SRPComm_GetResponseCode(c_this: *const c_void, Response: *mut i8, RspInfo: *mut c_char, RspInfoSize: i32) -> i32;
    fn Star_SRPComm_GetResponseStr(c_this: *const c_void, Response: *mut i8, Title: *const c_char, Buf: *mut c_char, BufSize: i32) -> VS_BOOL;
    fn Star_SRPComm_GetResponseBody(c_this: *const c_void, Response: *mut i8) -> *mut i8;
    fn Star_SRPComm_HttpLocalRequest(c_this: *const c_void, MsgHandle: *const c_void, ConnectionGroup: u32, MsgClass: u16, RequestType: u8, BoundaryNumber: u8, FileSize: u64, FileName: *const c_char, ContentType: *const c_char, Cookie: *const c_char, BoundaryInfo: *mut u32, RequestHeader: *const c_char, RequestBody: *const c_char) -> u32;
    fn Star_SRPComm_HttpLocalRequestEx(c_this: *const c_void, MsgHandle: *const c_void, ConnectionGroup: u32, MsgClass: u16, HtmlRequestPlain: *const c_char) -> u32;
    fn Star_SRPComm_SetupTimer(c_this: *const c_void, Interval: i32, NumberOfValid: i32, MsgHandle: *const c_void, ConnectionGroup: u32, MsgClass: u16) -> u32;
    fn Star_SRPComm_KillTimer(c_this: *const c_void, TimerID: u32);
    fn Star_SRPComm_Dup(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPComm_HookManagerMsgProc(c_this: *const c_void, ManagerMsgProc: *const c_void);
    fn Star_SRPComm_UnHookManagerMsgProc(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPComm_SetupManagerTimer(c_this: *const c_void, Interval: i32, NumberOfValid: i32, ConnectionGroup: u32, MsgClass: u16) -> u32;
    fn Star_SRPComm_FormatRspHeaderEx(c_this: *const c_void, RspInfo: *const c_char, ServerInfo: *const c_char, Connection: *const c_char, ContentType: *const c_char, ContentLength: u64, ExtendInfo: *const c_char, Buf: *mut c_char);
    fn Star_SRPComm_Http_GetHeaderItem(c_this: *const c_void, Header: *const i8, ItemIndex: i32, ItemName: *const c_char, RetBuf: *mut c_char, RetBufSize: i32) -> VS_BOOL;
    fn Star_SRPComm_Http_GetHeaderSubItem(c_this: *const c_void, Item: *const c_char, SubItemIndex: i32, SubItemName: *const c_char, RetBuf: *mut c_char, RetBufSize: i32) -> VS_BOOL;
    fn Star_SRPComm_Http_GetNVValue(c_this: *const c_void, Buf: *const c_char, Name: *const c_char, RetBuf: *mut c_char, RetBufSize: i32) -> VS_BOOL;
    fn Star_SRPComm_Http_TimeToHttpTime(c_this: *const c_void, Time: *mut VS_TIME, HttpTimeBuf: *mut c_char);
    fn Star_SRPComm_Http_HttpTimeToTime(c_this: *const c_void, HttpTimeBuf: *mut c_char, Time: *mut VS_TIME);
    fn Star_SRPComm_Http_SetCookie(c_this: *const c_void, Domain: *const c_char, Path: *const c_char, Cookie: *const c_char, Secure: VS_BOOL);
    fn Star_SRPComm_Http_ClearCookie(c_this: *const c_void, Domain: *const c_char, Path: *const c_char, Cookie: *const c_char);
    fn Star_SRPComm_Http_GetMediaType(c_this: *const c_void, FileName: *const c_char) -> *mut c_char;
    fn Star_SRPComm_Http_SetMaxPostSize(c_this: *const c_void, ConnectionID: u32, MaxPostSize: u32);
    fn Star_SRPComm_Http_GetMultiPart(c_this: *const c_void, RequestBody: *mut i8, Index: u8, BoundaryNumber: u8, BoundaryInfo: *mut u32, RetMultiPartBodySize: *mut u64, RetBuf: *mut c_char, RetBufSize: i32) -> *mut i8;
    fn Star_SRPComm_LuaPushHttpOnRequest(c_this: *const c_void, ServiceGroupIndex: u32, HttpOnRequest: *mut StructOfSRPComm_HttpOnRequest);
    fn Star_SRPComm_IsTCPConnect(c_this: *const c_void, ConnectionID: u32) -> VS_BOOL;
    fn Star_SRPComm_IsHttpConnect(c_this: *const c_void, ConnectionID: u32) -> VS_BOOL;
    fn Star_SRPComm_FileDownLoad(c_this: *const c_void, Url: *const c_char, LocalFileName: *const c_char, WaitFlag: VS_BOOL, CallBackProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPComm_FileUpLoad(c_this: *const c_void, Url: *const c_char, LocalFileName: *const c_char, RemoteFileName: *const c_char, RetBinBuf: *mut c_void, MultiPartFlag: VS_BOOL, ContentType: *const c_char, WaitFlag: VS_BOOL, CallBackProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPComm_BufDownLoad(c_this: *const c_void, Url: *const c_char, BinBuf: *mut c_void, WaitFlag: VS_BOOL, CallBackProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPComm_BufUpLoad(c_this: *const c_void, Url: *const c_char, BinBuf: *mut c_void, RemoteFileName: *const c_char, RetBinBuf: *mut c_void, MultiPartFlag: VS_BOOL, ContentType: *const c_char, WaitFlag: VS_BOOL, CallBackProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPComm_AddRef(c_this: *const c_void);
    fn Star_SRPComm_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPComm_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPSXML_Release(c_this: *const c_void);
    fn Star_SRPSXML_LoadFromFile(c_this: *const c_void, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPSXML_LoadFromBuf(c_this: *const c_void, Buf: *const i8, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPSXML_SaveToFile(c_this: *const c_void, FileName: *const c_char) -> VS_BOOL;
    fn Star_SRPSXML_SaveToBuf(c_this: *const c_void, BinBuf: *mut c_void) -> VS_BOOL;
    fn Star_SRPSXML_GetStandalone(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPSXML_GetVersion(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPSXML_GetEncoding(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPSXML_FindElement(c_this: *const c_void, Value: *const c_char) -> *mut c_void;
    fn Star_SRPSXML_FindElementEx(c_this: *const c_void, ParentElement: *mut c_void, Value: *const c_char) -> *mut c_void;
    fn Star_SRPSXML_FirstElement(c_this: *const c_void, ParentElement: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_NextElement(c_this: *const c_void, Element: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_ParentElement(c_this: *const c_void, Element: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_GetElement(c_this: *const c_void, Element: *mut c_void) -> *mut c_char;
    fn Star_SRPSXML_GetElementEx(c_this: *const c_void, Element: *mut c_void, Buf: *mut c_char, BufSize: i32);
    fn Star_SRPSXML_GetNs(c_this: *const c_void, Element: *mut c_void, nsName: *mut c_char, nsNameBufSize: i32, nsValue: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPSXML_GetNsValue(c_this: *const c_void, Element: *mut c_void, nsName: *const c_char) -> *mut c_char;
    fn Star_SRPSXML_SetNs(c_this: *const c_void, Element: *mut c_void, nsName: *const c_char, nsValue: *const c_char);
    fn Star_SRPSXML_FindAttribute(c_this: *const c_void, Element: *mut c_void, Name: *const c_char) -> *mut c_void;
    fn Star_SRPSXML_FirstAttribute(c_this: *const c_void, Element: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_NextAttribute(c_this: *const c_void, Attribute: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_GetAttributeName(c_this: *const c_void, Attribute: *mut c_void) -> *mut c_char;
    fn Star_SRPSXML_GetAttributeValue(c_this: *const c_void, Attribute: *mut c_void) -> *mut c_char;
    fn Star_SRPSXML_GetSingleText(c_this: *const c_void, Element: *mut c_void) -> *mut c_char;
    fn Star_SRPSXML_FirstText(c_this: *const c_void, Element: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_NextText(c_this: *const c_void, Text: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_GetText(c_this: *const c_void, Text: *mut c_void) -> *mut c_char;
    fn Star_SRPSXML_SetDeclaration(c_this: *const c_void, Version: *const c_char, Encoding: *const c_char, Standalone: *const c_char);
    fn Star_SRPSXML_RemoveDeclaration(c_this: *const c_void);
    fn Star_SRPSXML_InsertElementBefore(c_this: *const c_void, ParentElement: *mut c_void, Element: *mut c_void, Value: *const c_char) -> *mut c_void;
    fn Star_SRPSXML_InsertElementAfter(c_this: *const c_void, ParentElement: *mut c_void, Element: *mut c_void, Value: *const c_char) -> *mut c_void;
    fn Star_SRPSXML_RemoveElement(c_this: *const c_void, Element: *mut c_void);
    fn Star_SRPSXML_SetElement(c_this: *const c_void, Element: *mut c_void, Value: *const c_char);
    fn Star_SRPSXML_InsertTextBefore(c_this: *const c_void, ParentElement: *mut c_void, Text: *mut c_void, Value: *const c_char, CDataFlag: VS_BOOL) -> *mut c_void;
    fn Star_SRPSXML_InsertTextAfter(c_this: *const c_void, ParentElement: *mut c_void, Text: *mut c_void, Value: *const c_char, CDataFlag: VS_BOOL) -> *mut c_void;
    fn Star_SRPSXML_RemoveText(c_this: *const c_void, Text: *mut c_void);
    fn Star_SRPSXML_SetText(c_this: *const c_void, Text: *mut c_void, Value: *const c_char, CDataFlag: VS_BOOL);
    fn Star_SRPSXML_InsertCommentBefore(c_this: *const c_void, ParentElement: *mut c_void, Comment: *mut c_void, Value: *const c_char) -> *mut c_void;
    fn Star_SRPSXML_InsertCommentAfter(c_this: *const c_void, ParentElement: *mut c_void, Comment: *mut c_void, Value: *const c_char) -> *mut c_void;
    fn Star_SRPSXML_RemoveComment(c_this: *const c_void, Comment: *mut c_void);
    fn Star_SRPSXML_SetComment(c_this: *const c_void, Comment: *mut c_void, Value: *const c_char);
    fn Star_SRPSXML_SetAttribute(c_this: *const c_void, Element: *mut c_void, Name: *const c_char, Value: *const c_char);
    fn Star_SRPSXML_RemoveAttribute(c_this: *const c_void, Element: *mut c_void, Name: *const c_char);
    fn Star_SRPSXML_GetRow(c_this: *const c_void, Node: *mut c_void) -> i32;
    fn Star_SRPSXML_GetCol(c_this: *const c_void, Node: *mut c_void) -> i32;
    fn Star_SRPSXML_Dup(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPSXML_Copy(c_this: *const c_void, SrcSXML: *mut c_void) -> VS_BOOL;
    fn Star_SRPSXML_CopyElementBefore(c_this: *const c_void, ParentElement: *mut c_void, Element: *mut c_void, SrcElement: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_CopyElementAfter(c_this: *const c_void, ParentElement: *mut c_void, Element: *mut c_void, SrcElement: *mut c_void) -> *mut c_void;
    fn Star_SRPSXML_CopyChild(c_this: *const c_void, DesElement: *mut c_void, SrcElement: *mut c_void) -> VS_BOOL;
    fn Star_SRPSXML_LoadFromAnsiBuf(c_this: *const c_void, Buf: *const i8, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPSXML_SaveToAnsiBuf(c_this: *const c_void, BinBuf: *mut c_void) -> VS_BOOL;
    fn Star_SRPSXML_AddRef(c_this: *const c_void);
    fn Star_SRPSXML_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPSXML_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPControl_Release(c_this: *const c_void);
    fn Star_SRPControl_SRPLock(c_this: *const c_void);
    fn Star_SRPControl_SRPUnLock(c_this: *const c_void);
    fn Star_SRPControl_GetOsType(c_this: *const c_void) -> u32;
    fn Star_SRPControl_SRPDispatch(c_this: *const c_void, WaitFlag: VS_BOOL) -> VS_BOOL;
    fn Star_SRPControl_SRPIdle(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPControl_SRPAppActive(c_this: *const c_void);
    fn Star_SRPControl_SRPAppDeactive(c_this: *const c_void);
/*  fn Star_SRPControl_ProcessError(); */
/*  fn Star_SRPControl_ProcessErrorVar(); */
/*  fn Star_SRPControl_ProcessLuaError(); */
/*  fn Star_SRPControl_ProcessLuaErrorVar(); */
    fn Star_SRPControl_IsAppActive(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPControl_CaptureLuaDisp(c_this: *const c_void, DispProc: *mut c_void, Para: usize);
    fn Star_SRPControl_ReleaseLuaDisp(c_this: *const c_void, DispProc: *mut c_void, Para: usize);
    fn Star_SRPControl_PreCompile(c_this: *const c_void, ScriptInterface: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, Name: *const c_char, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPControl_OpenLuaEdit(c_this: *const c_void, Module: *const c_char, Config: u32, CloseEnable: VS_BOOL) -> VS_BOOL;
    fn Star_SRPControl_LuaEditDisp(c_this: *const c_void, Info: *const c_char);
    fn Star_SRPControl_CloseLuaEdit(c_this: *const c_void);
    fn Star_SRPControl_ClearService(c_this: *const c_void);
    fn Star_SRPControl_DoBuffer(c_this: *const c_void, ScriptInterface: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, IsUTF8: VS_BOOL, ModuleName: *const c_char) -> VS_BOOL;
    fn Star_SRPControl_PostDoBuffer(c_this: *const c_void, ScriptInterface: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, IsUTF8: VS_BOOL, ModuleName: *const c_char);
    fn Star_SRPControl_DoFile(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL) -> VS_BOOL;
    fn Star_SRPControl_PostDoFile(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL) -> VS_BOOL;
    fn Star_SRPControl_LuaEditHelp(c_this: *const c_void, Type: i32, HelpInfo: *const c_char);
    fn Star_SRPControl_QueryBasicInterface(c_this: *const c_void, ServiceGroupID: u32) -> *mut c_void;
    fn Star_SRPControl_CreateBasicInterface(c_this: *const c_void, ServiceGroupID: u32, ProgramRunType: u16) -> *mut c_void;
    fn Star_SRPControl_DeleteBasicInterface(c_this: *const c_void, ServiceGroupID: u32);
    fn Star_SRPControl_QueryFirstServiceGroup(c_this: *const c_void) -> u32;
    fn Star_SRPControl_QueryNextServiceGroup(c_this: *const c_void) -> u32;
    fn Star_SRPControl_GetLuaStack(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPControl_LuaGetTableInt(c_this: *const c_void, L: *mut c_void, Index: i32, ValueName: *const c_char) -> u32;
    fn Star_SRPControl_LuaGetInt(c_this: *const c_void, L: *mut c_void, Index: i32) -> u32;
    fn Star_SRPControl_LuaUpValueIndex(c_this: *const c_void, L: *mut c_void, Index: i32) -> i32;
    fn Star_SRPControl_GetObjectServiceGroupID(c_this: *const c_void, Object: *mut c_void) -> u32;
    fn Star_SRPControl_GetUrl(c_this: *const c_void, Buf: *mut c_char, BufSize: i32);
    fn Star_SRPControl_SetProgramType(c_this: *const c_void, Type: u16);
    fn Star_SRPControl_GetProgramType(c_this: *const c_void) -> u16;
    fn Star_SRPControl_SRPBuild_QueryPublicService(c_this: *const c_void, QueryPublicServiceCallBackProc: *mut c_void, CallBackPara: usize, FillUpdateInfo: VS_BOOL, PrintProc: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPControl_SRPBuild_QueryPublicServiceEx(c_this: *const c_void, Url: *const c_char, QueryPublicServiceCallBackProc: *mut c_void, CallBackPara: usize, FillUpdateInfo: VS_BOOL, PrintProc: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPControl_SRPBuild_Start(c_this: *const c_void, Name: *const c_char, Path: *const c_char, SingleFlag: VS_BOOL, ForceToDownLoadPublicService: VS_BOOL, PrintProc: *mut c_void, Para: usize, PublicServiceList: *mut VSPublicServiceDef, ExeFileFlag: VS_BOOL, ScriptInterface: *const c_char, SupportOsType: u32) -> VS_BOOL;
    fn Star_SRPControl_SRPBuild_InsertServiceFile(c_this: *const c_void, DiskFileName: *const c_char, OutFileName: *const c_char, StartFileFlag: VS_BOOL, ToUTF8: VS_BOOL, SupportOsType: u32);
    fn Star_SRPControl_SRPBuild_InsertDependFile(c_this: *const c_void, Path: *const c_char, DependName: *const c_char);
    fn Star_SRPControl_SRPBuild_InsertStaticDataFile(c_this: *const c_void, DiskFileName: *const c_char, OutFileName: *const c_char, ToUTF8: VS_BOOL);
    fn Star_SRPControl_SRPBuild_InsertDynaDataFile(c_this: *const c_void, DiskFileName: *const c_char, OutFileName: *const c_char, ToUTF8: VS_BOOL);
    fn Star_SRPControl_SRPBuild_Execute(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPControl_StringToUuid(c_this: *const c_void, String: *const i8, Uuid: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPControl_UuidToString(c_this: *const c_void, Uuid: *mut VS_UUID) -> *mut i8;
    fn Star_SRPControl_MD5ToUuid(c_this: *const c_void, String: *const i8, Uuid: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPControl_UuidToMD5(c_this: *const c_void, Uuid: *mut VS_UUID) -> *mut i8;
    fn Star_SRPControl_GetMD5(c_this: *const c_void, Buf: *mut i8, BufSize: i32) -> *mut i8;
    fn Star_SRPControl_CreateUuid(c_this: *const c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPControl_GetSXMLInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPControl_GetCommInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPControl_GetCoreShellInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPControl_RegScriptInterface(c_this: *const c_void, ScriptInterface: *const c_char, ScriptContext: *mut StructOfVSScriptContext, Para: usize, TermProc: *mut c_void) -> VS_BOOL;
    fn Star_SRPControl_UnRegScriptInterface(c_this: *const c_void, ScriptInterface: *const c_char, ScriptContext: *mut StructOfVSScriptContext, Para: usize, TermProc: *mut c_void) -> VS_BOOL;
    fn Star_SRPControl_ActiveScriptInterface(c_this: *const c_void, ScriptInterface: *const c_char, OnLineScriptFlag: *mut VS_BOOL, VirtualMachine: *mut c_void) -> VS_BOOL;
    fn Star_SRPControl_FirstScriptInterface(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_char;
    fn Star_SRPControl_NextScriptInterface(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_char;
    fn Star_SRPControl_RegTempFile(c_this: *const c_void, TempFileName: *const c_char, OriFileName: *const c_char) -> VS_BOOL;
    fn Star_SRPControl_GetRegTempFile(c_this: *const c_void, OriFileName: *const c_char, Buf: *mut c_char, BufSize: i32) -> *mut c_char;
    fn Star_SRPControl_UnRegTempFile(c_this: *const c_void, TempFileName: *const c_char);
    fn Star_SRPControl_GetConfigResult(c_this: *const c_void, DebugCfgResult: *mut VS_BOOL, DirectClientCfgResult: *mut VS_BOOL, TelnetCfgResult: *mut VS_BOOL, WebServerCfgResult: *mut VS_BOOL);
    fn Star_SRPControl_GetConfigEnvTag(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPControl_Dup(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPControl_QueryInterface(c_this: *const c_void, InterfaceID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPControl_AddRef(c_this: *const c_void);
    fn Star_SRPControl_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPControl_SetLogFile(c_this: *const c_void, FileName: *const c_char, LogAll: VS_BOOL);
    fn Star_SRPControl_GetSystemRegCode(c_this: *const c_void, Buf: *mut c_char);
    fn Star_SRPControl_SetRegisterCode(c_this: *const c_void, Buf: *const c_char, Single: VS_BOOL) -> VS_BOOL;
    fn Star_SRPControl_IsRegistered(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPControl_GetHashValue(c_this: *const c_void, Key: *mut c_void, Length: u32, InitValue: u32) -> u32;
    fn Star_SRPControl_ScriptStarcoreRecord(c_this: *const c_void, ScriptInterface: *const c_char, SharelibHandle: *const c_void);
    fn Star_SRPControl_ScriptStarcoreUnRecord(c_this: *const c_void, ScriptInterface: *const c_char, SharelibHandle: *const c_void);
    fn Star_SRPControl_ScriptStarcoreIsRecord(c_this: *const c_void, ScriptInterface: *const c_char) -> VS_BOOL;
    fn Star_SRPControl_FindWindow(c_this: *const c_void, Caption: *const c_char) -> usize;
    fn Star_SRPControl_WinMsgLoop(c_this: *const c_void, QuitFlag: *mut VS_BOOL) -> VS_BOOL;
    fn Star_SRPControl_RegScriptObject(c_this: *const c_void, ScriptObject: usize, FreeScriptObjectProc: *mut c_void, Para: usize);
    fn Star_SRPControl_UnRegScriptObject(c_this: *const c_void, ScriptObject: usize, FreeScriptObjectProc: *mut c_void, Para: usize);
    fn Star_SRPControl_ClearScriptObject(c_this: *const c_void, FreeScriptObjectProc: *mut c_void, Para: usize);
    fn Star_SRPControl_PreAuthorize(c_this: *const c_void, ServiceName: *const c_char, ServiceID: *mut VS_UUID, Buf: *const c_char, Single: VS_BOOL) -> VS_BOOL;
    fn Star_SRPControl_SetLocale(c_this: *const c_void, Lang: *const c_char);
    fn Star_SRPControl_GetLocale(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPControl_SetScriptInterfaceIndex(c_this: *const c_void, ScriptInterfaceName: *const c_char);
    fn Star_SRPControl_GetScriptInterfaceIndex(c_this: *const c_void, ScriptInterfaceName: *const c_char) -> i32;
    fn Star_SRPControl_DetachCurrentThread(c_this: *const c_void);
    fn Star_SRPControl_DoFileEx(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL, ModuleName: *const c_char) -> VS_BOOL;
    fn Star_SRPControl_PostDoFileEx(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL, ModuleName: *const c_char) -> VS_BOOL;
    fn Star_SRPControl_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPControl_GetLastError(c_this: *const c_void) -> i32;
    fn Star_SRPControl_GetLastErrorInfo(c_this: *const c_void, LineIndex: *mut u32, SourceName: *mut [*mut c_char;1]) -> *mut c_char;
    fn Star_SRPControl_SetScriptInterface(c_this: *const c_void, ScriptInterface: *const c_char, Module: *const c_char, Para: *const c_char) -> VS_BOOL;
    fn Star_SRPControl_SetCoreOperationPath(c_this: *const c_void, Path: *const c_char) -> *mut c_char;
    fn Star_SRPControl_GetCFunctionTable(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPControl_LuaGetTableInt64(c_this: *const c_void, L: *mut c_void, Index: i32, ValueName: *const c_char) -> u64;
    fn Star_SRPControl_LuaGetInt64(c_this: *const c_void, L: *mut c_void, Index: i32) -> u64;
    fn Star_SRPControl_LuaGetTableUWord(c_this: *const c_void, L: *mut c_void, Index: i32, ValueName: *const c_char) -> usize;
    fn Star_SRPControl_LuaGetUWord(c_this: *const c_void, L: *mut c_void, Index: i32) -> usize;
    fn Star_SRPControl_RegisterProc(c_this: *const c_void, FunctionName: *mut c_char, FunctionProc: *mut c_void) -> *mut c_void;
    fn Star_SRPControl_RunInMainThread(c_this: *const c_void, CallBack: *mut c_void, Para1: usize, Para2: usize, Para3: usize, Para4: usize) -> usize;
    fn Star_SRPControl_RegDispatchRequest(c_this: *const c_void, CallBack: *mut c_void, Para: usize);
    fn Star_SRPControl_UnRegDispatchRequest(c_this: *const c_void, CallBack: *mut c_void, Para: usize);
    fn Star_SRPBasic_Release(c_this: *const c_void);
    fn Star_SRPBasic_GetOsType(c_this: *const c_void) -> u32;
/*  fn Star_SRPBasic_Print(); */
/*  fn Star_SRPBasic_PrintVar(); */
/*  fn Star_SRPBasic_PrintLua(); */
/*  fn Star_SRPBasic_PrintLuaVar(); */
    fn Star_SRPBasic_SetPrintToLua(c_this: *const c_void, PrintFlag: VS_BOOL);
/*  fn Star_SRPBasic_MessageBox(); */
/*  fn Star_SRPBasic_MessageBoxVar(); */
/*  fn Star_SRPBasic_ProcessError(); */
/*  fn Star_SRPBasic_ProcessErrorVar(); */
/*  fn Star_SRPBasic_ProcessLuaError(); */
/*  fn Star_SRPBasic_ProcessLuaErrorVar(); */
    fn Star_SRPBasic_CaptureLuaDisp(c_this: *const c_void, DispProc: *mut c_void, Para: usize);
    fn Star_SRPBasic_ReleaseLuaDisp(c_this: *const c_void, DispProc: *mut c_void, Para: usize);
    fn Star_SRPBasic_SetDefaultPath(c_this: *const c_void, DefaultPath: *const c_char);
    fn Star_SRPBasic_GetDefaultPath(c_this: *const c_void, DefaultPath: *mut c_char, BufSize: i32);
    fn Star_SRPBasic_DefaultPathIsSet(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_Connect(c_this: *const c_void, ServerInterface: *const c_char, ServerName: *const c_char, ServerPortNumber: u16, RetrySecond: i32, ParaPkg: *mut c_void, ClientConnectCallBack: *mut c_void, Para: usize, LoginName: *const c_char, LoginPassword: *const c_char) -> u32;
    fn Star_SRPBasic_ConnectEx(c_this: *const c_void, ServiceName: *const c_char, RetrySecond: i32, ParaPkg: *mut c_void, ClientConnectCallBack: *mut c_void, Para: usize, LoginName: *const c_char, LoginPassword: *const c_char) -> u32;
    fn Star_SRPBasic_SConnect(c_this: *const c_void, ServerInterface: *const c_char, ServerName: *const c_char, ServerPortNumber: u16, ParaPkg: *mut c_void, LoginName: *const c_char, LoginPassword: *const c_char) -> u32;
    fn Star_SRPBasic_SConnectEx(c_this: *const c_void, ServiceName: *const c_char, ParaPkg: *mut c_void, LoginName: *const c_char, LoginPassword: *const c_char) -> u32;
    fn Star_SRPBasic_Connect2(c_this: *const c_void, ServerInterface: *const c_char, ServerName: *const c_char, ServerPortNumber: u16, ParaPkg: *mut c_void, LoginName: *const c_char, LoginPassword: *const c_char, SysRootItemName: *const c_char) -> *mut c_void;
    fn Star_SRPBasic_ConnectEx2(c_this: *const c_void, ServiceName: *const c_char, ParaPkg: *mut c_void, LoginName: *const c_char, LoginPassword: *const c_char, SysRootItemName: *const c_char) -> *mut c_void;
    fn Star_SRPBasic_DisConnectEx(c_this: *const c_void, ConnectionID: u32);
    fn Star_SRPBasic_DisConnect(c_this: *const c_void);
    fn Star_SRPBasic_IsConnect(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_GetLua(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_DoBuffer(c_this: *const c_void, ScriptInterface: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, IsUTF8: VS_BOOL, ModuleName: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_LuaToBool(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaToString(c_this: *const c_void, Index: i32) -> *mut c_char;
    fn Star_SRPBasic_LuaToNumber(c_this: *const c_void, Index: i32) -> f64;
    fn Star_SRPBasic_LuaToInt(c_this: *const c_void, Index: i32) -> i32;
    fn Star_SRPBasic_LuaIsBool(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaIsString(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaIsNumber(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaGetGlobal(c_this: *const c_void, Name: *const c_char);
    fn Star_SRPBasic_LuaGetSrvGroupTable(c_this: *const c_void, Name: *const c_char);
    fn Star_SRPBasic_LuaPop(c_this: *const c_void, Index: i32);
    fn Star_SRPBasic_LuaPushBool(c_this: *const c_void, Value: VS_BOOL);
    fn Star_SRPBasic_LuaPushString(c_this: *const c_void, Value: *const c_char);
    fn Star_SRPBasic_LuaPushNumber(c_this: *const c_void, Value: f64);
    fn Star_SRPBasic_LuaPushInt(c_this: *const c_void, Value: i32);
    fn Star_SRPBasic_LuaPushNil(c_this: *const c_void);
    fn Star_SRPBasic_LuaSetGlobal(c_this: *const c_void, Name: *const c_char);
    fn Star_SRPBasic_LuaSetSrvGroupTable(c_this: *const c_void, Name: *const c_char);
    fn Star_SRPBasic_LuaIsNil(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaNewTable(c_this: *const c_void);
    fn Star_SRPBasic_LuaIsTable(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaGetTop(c_this: *const c_void) -> i32;
    fn Star_SRPBasic_LuaObjLen(c_this: *const c_void, TableIndex: i32) -> i32;
    fn Star_SRPBasic_LuaPushLString(c_this: *const c_void, Value: *const c_char, Len: u32);
    fn Star_SRPBasic_LuaPushParaPkg(c_this: *const c_void, ParaPkg: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LuaPushQueryRecord(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LuaToParaPkg(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPBasic_LuaToQueryRecord(c_this: *const c_void, Index: i32) -> *mut VS_QUERYRECORD;
    fn Star_SRPBasic_LuaIsParaPkg(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaIsQueryRecord(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaPushBinBuf(c_this: *const c_void, BinBuf: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LuaToBinBuf(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPBasic_LuaIsBinBuf(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaPushSXml(c_this: *const c_void, SXml: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LuaToSXml(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPBasic_LuaIsSXml(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaPushFunctionPara(c_this: *const c_void, FunctionPara: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LuaToFunctionPara(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPBasic_LuaIsFunctionPara(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaPushCommInterface(c_this: *const c_void, CommInterface: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LuaToCommInterface(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPBasic_LuaIsCommInterface(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_ClearLuaGlobal(c_this: *const c_void);
    fn Star_SRPBasic_PreCompile(c_this: *const c_void, ScriptInterface: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, Name: *const c_char, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPBasic_GCCollect(c_this: *const c_void);
    fn Star_SRPBasic_PostDoBuffer(c_this: *const c_void, ScriptInterface: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, IsUTF8: VS_BOOL, ModuleName: *const c_char);
    fn Star_SRPBasic_PostDoBufferEx(c_this: *const c_void, ScriptInterface: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, DispProc: *const c_void, Para: usize, IsUTF8: VS_BOOL, ModuleName: *const c_char);
    fn Star_SRPBasic_DoFile(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_PostDoFile(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LuaRegHook(c_this: *const c_void, FuncAddr: *mut c_void);
    fn Star_SRPBasic_LuaUnRegHook(c_this: *const c_void, FuncAddr: *mut c_void);
    fn Star_SRPBasic_RegGCProc(c_this: *const c_void, GCProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPBasic_UnRegGCProc(c_this: *const c_void, GCProc: *const c_void, Para: usize);
    fn Star_SRPBasic_GetServiceGroupID(c_this: *const c_void) -> u32;
    fn Star_SRPBasic_QueryFirstService(c_this: *const c_void, RetUuid: *mut VS_UUID) -> *mut c_char;
    fn Star_SRPBasic_QueryNextService(c_this: *const c_void, RetUuid: *mut VS_UUID) -> *mut c_char;
    fn Star_SRPBasic_QueryActiveService(c_this: *const c_void, RetUuid: *mut VS_UUID) -> *mut c_char;
    fn Star_SRPBasic_GetServiceName(c_this: *const c_void, ServiceID: *mut VS_UUID) -> *mut c_char;
    fn Star_SRPBasic_GetServiceID(c_this: *const c_void, ServiceName: *const c_char, ServiceID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPBasic_IsValidUser(c_this: *const c_void, UserName: *const c_char, UserPass: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_ImportServiceEx(c_this: *const c_void, ServiceID: *mut VS_UUID, LoadRunModule: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_ImportServiceWithPath(c_this: *const c_void, ServicePath: *const c_char, ServiceName: *const c_char, LoadRunModule: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_ImportService(c_this: *const c_void, ServiceName: *const c_char, LoadRunModule: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_ImportDynaService(c_this: *const c_void, Url: *const c_char) -> *mut c_char;
    fn Star_SRPBasic_CreateService(c_this: *const c_void, ServicePath: *const c_char, ServiceName: *const c_char, ServiceID: *mut VS_UUID, RootPass: *const c_char, FrameInterval: i32, NetPkgSize: i32, UploadPkgSize: i32, DownloadPkgSize: i32, DataUpPkgSize: i32, DataDownPkgSize: i32) -> VS_BOOL;
    fn Star_SRPBasic_CreateServiceEx(c_this: *const c_void, ServicePath: *const c_char, ServiceName: *const c_char, ServiceID: *mut VS_UUID, RootPass: *const c_char, FrameInterval: i32, NetPkgSize: i32, UploadPkgSize: i32, DownloadPkgSize: i32, DataUpPkgSize: i32, DataDownPkgSize: i32) -> VS_BOOL;
    fn Star_SRPBasic_LoadServiceEx(c_this: *const c_void, ServiceID: *mut VS_UUID, UserName: *const c_char, UserPass: *const c_char, LoadRunModule: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LoadServiceWithPath(c_this: *const c_void, ServicePath: *const c_char, ServiceName: *const c_char, UserName: *const c_char, UserPass: *const c_char, LoadRunModule: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LoadService(c_this: *const c_void, ServiceName: *const c_char, UserName: *const c_char, UserPass: *const c_char, LoadRunModule: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_ExportServiceHeader(c_this: *const c_void, ServiceName: *const c_char, Path: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_ExportServiceDefine(c_this: *const c_void, ServiceName: *const c_char, FileName: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_ClearService(c_this: *const c_void);
    fn Star_SRPBasic_ClearServiceEx(c_this: *const c_void);
    fn Star_SRPBasic_RegisterServer(c_this: *const c_void, ServiceName: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_AllocCooperator(c_this: *const c_void, ServiceName: *const c_char);
    fn Star_SRPBasic_FreeCooperator(c_this: *const c_void, ServiceName: *const c_char);
    fn Star_SRPBasic_GetServerUrlInfo(c_this: *const c_void, ServerUrlInfo: *mut StructOfVSServerUrlInfo);
    fn Star_SRPBasic_WebServiceRefresh(c_this: *const c_void);
    fn Star_SRPBasic_GetWSDL(c_this: *const c_void, WSDLVersion: u32, WebServiceHost: *const c_char, BinBuf: *mut c_void) -> VS_BOOL;
    fn Star_SRPBasic_XmlToService(c_this: *const c_void, SXMLInterface: *mut c_void, DataPath: *const c_char, SegmentName: *const c_char, PrintProc: *const c_void, Para: usize) -> *mut c_void;
    fn Star_SRPBasic_GetSRPInterfaceEx(c_this: *const c_void, ServiceID: *mut VS_UUID, UserName: *const c_char, UserPass: *const c_char) -> *mut c_void;
    fn Star_SRPBasic_GetSRPInterface(c_this: *const c_void, ServiceName: *const c_char, UserName: *const c_char, UserPass: *const c_char) -> *mut c_void;
    fn Star_SRPBasic_GetSRPInterface2(c_this: *const c_void, ServiceName: *const c_char, CallBackProc: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetSRPInterfaceEx2(c_this: *const c_void, ServiceID: *mut VS_UUID, CallBackProc: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_ChangeDepend(c_this: *const c_void, OldDependServiceID: *mut VS_UUID, NewDependServiceID: *mut VS_UUID, NewServiceName: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_AddDepend(c_this: *const c_void, DependServiceID: *mut VS_UUID, NewServiceName: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_GetRequestBuf(c_this: *const c_void) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPBasic_FreeResponseBuf(c_this: *const c_void, ResponseParam: *mut VS_EVENTPARAM_RUNPARAM);
    fn Star_SRPBasic_FreeRequestBuf(c_this: *const c_void, RequestParam: *mut VS_EVENTPARAM_RUNPARAM);
    fn Star_SRPBasic_ProcessSysObjectEvent(c_this: *const c_void, EventID: *mut VS_UUID, RequestParam: *mut VS_EVENTPARAM_RUNPARAM) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPBasic_GetSysObject(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetName(c_this: *const c_void, ObjectID: *mut VS_UUID) -> *mut c_char;
    fn Star_SRPBasic_GetMD5(c_this: *const c_void, Buf: *mut i8, BufSize: i32) -> *mut i8;
    fn Star_SRPBasic_GetTickCount(c_this: *const c_void) -> usize;
    fn Star_SRPBasic_GetID(c_this: *const c_void, Object: *mut c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPBasic_RegObjectIDChangeNotify(c_this: *const c_void, ChangeNotifyProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPBasic_UnRegObjectIDChangeNotify(c_this: *const c_void, ChangeNotifyProc: *const c_void, Para: usize);
    fn Star_SRPBasic_RegObjectFreeNotify(c_this: *const c_void, FreeNotifyProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPBasic_UnRegObjectFreeNotify(c_this: *const c_void, FreeNotifyProc: *const c_void, Para: usize);
    fn Star_SRPBasic_GetRegStr(c_this: *const c_void, SubKey: *const c_char, ValueName: *const c_char, DefaultValue: *const c_char) -> *mut c_char;
    fn Star_SRPBasic_GetRegInt(c_this: *const c_void, SubKey: *const c_char, ValueName: *const c_char, DefaultValue: u32) -> u32;
    fn Star_SRPBasic_SetupTimer(c_this: *const c_void, Ticket: i32, FunctionAddr: *const c_void, Para1: usize, Para2: usize, Para3: usize, Para4: usize) -> u32;
    fn Star_SRPBasic_KillTimer(c_this: *const c_void, TimerID: u32);
    fn Star_SRPBasic_SetupSocketServer(c_this: *const c_void, Interface: *const c_char, PortNumber: u16, LinkInterfaceStatus: *mut u32, CallBackProc: *const c_void, Para: usize) -> u32;
    fn Star_SRPBasic_SetupSocketClient(c_this: *const c_void, ServerInterface: *const c_char, ServerName: *const c_char, PortNumber: u16, CallBackProc: *const c_void, Para: usize) -> u32;
    fn Star_SRPBasic_CloseSocketConnect(c_this: *const c_void, ConnectionID: u32);
    fn Star_SRPBasic_SocketSend(c_this: *const c_void, MachineID: u32, ParaPkg: *mut c_void, Assure: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_SocketSendBin(c_this: *const c_void, MachineID: u32, BinDataSize: i32, BinDataBuf: *mut i8, Assure: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_SetupSocketTimer(c_this: *const c_void, MachineID: u32, Ticket: i32, Counter: i32) -> u32;
    fn Star_SRPBasic_KillSocketTimer(c_this: *const c_void, MachineID: u32, TimerIndex: u32);
    fn Star_SRPBasic_SetDataServerAddr(c_this: *const c_void, DirectConnect: VS_BOOL, DataServerInterface: *const c_char, DataServerName: *const c_char, DataServerPort: u16, LocalDataServerInterface: *const c_char, LocalDataServerPort: u16) -> VS_BOOL;
    fn Star_SRPBasic_RegQueryStaticDataProc(c_this: *const c_void, Proc: *const c_void, Para: usize);
    fn Star_SRPBasic_UnRegQueryStaticDataProc(c_this: *const c_void, Proc: *const c_void, Para: usize);
    fn Star_SRPBasic_RegSaveStaticDataProc(c_this: *const c_void, Proc: *const c_void, Para: usize);
    fn Star_SRPBasic_UnRegSaveStaticDataProc(c_this: *const c_void, Proc: *const c_void, Para: usize);
    fn Star_SRPBasic_RegClearStaticDataProc(c_this: *const c_void, Proc: *const c_void, Para: usize);
    fn Star_SRPBasic_UnRegClearStaticDataProc(c_this: *const c_void, Proc: *const c_void, Para: usize);
    fn Star_SRPBasic_SetServerPara(c_this: *const c_void, MaxClientNumber: i32, MaxDataServerConnectionNumber: i32, DataServerOverTime: i32);
    fn Star_SRPBasic_QuyeryStatisticInfo(c_this: *const c_void, Machine: *mut c_void, InfoBuf: *mut VS_STATISTICINFO);
    fn Star_SRPBasic_RegFileCallBack(c_this: *const c_void, ServiceID: *mut VS_UUID, FileCallBackProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPBasic_UnRegFileCallBack(c_this: *const c_void, ServiceID: *mut VS_UUID, FileCallBackProc: *const c_void, Para: usize);
    fn Star_SRPBasic_RegFileReqCallBack(c_this: *const c_void, FileCallBackProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPBasic_UnRegFileReqCallBack(c_this: *const c_void, FileCallBackProc: *const c_void, Para: usize);
    fn Star_SRPBasic_GetFileInfo(c_this: *const c_void, InfoPtr: *mut VS_UPDOWNFILEINFO);
    fn Star_SRPBasic_StringToUuid(c_this: *const c_void, String: *const i8, Uuid: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPBasic_UuidToString(c_this: *const c_void, Uuid: *mut VS_UUID) -> *mut i8;
    fn Star_SRPBasic_GetProgramType(c_this: *const c_void) -> u16;
    fn Star_SRPBasic_IsDefaultServer(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_IsWindowVisible(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_HideWindow(c_this: *const c_void);
    fn Star_SRPBasic_ShowWindow(c_this: *const c_void);
    fn Star_SRPBasic_SetWindowCaption(c_this: *const c_void, Caption: *const c_char);
    fn Star_SRPBasic_ExitVSSystem(c_this: *const c_void, ErrorInfo: *const c_char);
    fn Star_SRPBasic_IsAppActive(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_SetIdleActive(c_this: *const c_void, CreateFlag: VS_BOOL);
    fn Star_SRPBasic_GetVersion(c_this: *const c_void, MainVersion: *mut u8, SubVersion: *mut u8, BuildVersion: *mut u16);
    fn Star_SRPBasic_GetVersionInfo(c_this: *const c_void, InfoBuf: *mut c_char, InfoBufSize: i32);
    fn Star_SRPBasic_GetWindowHandle(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetWindowSize(c_this: *const c_void, Width: *mut i32, Height: *mut i32);
    fn Star_SRPBasic_SetColor(c_this: *const c_void, Text: u32, Explane: u32, ObjName: u32, AttrType: u32, Number: u32, Error: u32);
    fn Star_SRPBasic_SetBkColor(c_this: *const c_void, BkColor: u32);
    fn Star_SRPBasic_ShowStatusMenu(c_this: *const c_void, MenuShowFlag: VS_BOOL, StatusShowFlag: VS_BOOL);
    fn Star_SRPBasic_GetClientWndHandle(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetClientWndSize(c_this: *const c_void, Width: *mut i32, Height: *mut i32);
    fn Star_SRPBasic_SetClientWndSize(c_this: *const c_void, Width: i32, Height: i32);
    fn Star_SRPBasic_SetClientWndFocus(c_this: *const c_void, hWnd: *mut c_void, NeedAction: VS_BOOL);
    fn Star_SRPBasic_KillClientWndFocus(c_this: *const c_void, hWnd: *mut c_void, NeedAction: VS_BOOL);
    fn Star_SRPBasic_ClearClientWnd(c_this: *const c_void);
    fn Star_SRPBasic_HideClientWnd(c_this: *const c_void);
    fn Star_SRPBasic_ShowClientWnd(c_this: *const c_void);
    fn Star_SRPBasic_SetClientBkColor(c_this: *const c_void, BkColor: u32);
    fn Star_SRPBasic_GetKeyState(c_this: *const c_void, Key: i32) -> VS_BOOL;
    fn Star_SRPBasic_SetWindowStyle(c_this: *const c_void, Style: *mut VSWINDOWSTYLE);
    fn Star_SRPBasic_MoveWindow(c_this: *const c_void, Position: *mut VSWINDOWPOSITION, RepaintFlag: VS_BOOL);
    fn Star_SRPBasic_GetWindowPos(c_this: *const c_void, Position: *mut VSWINDOWPOSITION);
    fn Star_SRPBasic_SetWindowStatus(c_this: *const c_void, Status: i32);
    fn Star_SRPBasic_RegRunEnv_FromParentCallBack(c_this: *const c_void, CallBack: *const c_void, Para: usize);
    fn Star_SRPBasic_UnRegRunEnv_FromParentCallBack(c_this: *const c_void, CallBack: *const c_void, Para: usize);
    fn Star_SRPBasic_RunEnvToChild(c_this: *const c_void, ObjectID: *mut VS_UUID, RunEnvInfo: *mut StructOfVSRunEnv) -> VS_BOOL;
    fn Star_SRPBasic_RunEnvToParent(c_this: *const c_void, RunEnvInfo: *mut StructOfVSRunEnv) -> VS_BOOL;
    fn Star_SRPBasic_SetMessageHook(c_this: *const c_void, HookProc: *const c_void);
    fn Star_SRPBasic_GetMessageHook(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_IsInSync(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_IsServiceSync(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_WaitServiceSync(c_this: *const c_void, WaitTimeMs: i32) -> VS_BOOL;
    fn Star_SRPBasic_SRPIdle(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_SRPAppActive(c_this: *const c_void);
    fn Star_SRPBasic_SRPAppDeactive(c_this: *const c_void);
    fn Star_SRPBasic_HyperLink(c_this: *const c_void, HyperLink: *const c_char, CreateNewWindow: VS_BOOL);
    fn Star_SRPBasic_AppEvent(c_this: *const c_void, EventID: u32, EventInfo: *const c_char);
    fn Star_SRPBasic_SetExceptHandler(c_this: *const c_void, ExceptHandlerProc: *const c_void);
    fn Star_SRPBasic_CreateIndex_Nor(c_this: *const c_void, KeyNumber: i32, HashTableBits: u16) -> *mut c_void;
    fn Star_SRPBasic_CreateIndexCmp_Nor(c_this: *const c_void, KeyNumber: i32, HashTableBits: u16, CompareProc: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_CreateIDIndex_Nor(c_this: *const c_void, HashTableBits: u16) -> *mut c_void;
    fn Star_SRPBasic_CreateIDIndexEx_Nor(c_this: *const c_void, HashTableBits: u16) -> *mut c_void;
    fn Star_SRPBasic_CreateIndex_Dbg(c_this: *const c_void, KeyNumber: i32, HashTableBits: u16, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPBasic_CreateIndexCmp_Dbg(c_this: *const c_void, KeyNumber: i32, HashTableBits: u16, CompareProc: *const c_void, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPBasic_CreateIDIndex_Dbg(c_this: *const c_void, HashTableBits: u16, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPBasic_CreateIDIndexEx_Dbg(c_this: *const c_void, HashTableBits: u16, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPBasic_InsertOneKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, Buf: *mut i8);
    fn Star_SRPBasic_FindOneKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize) -> *mut i8;
    fn Star_SRPBasic_DelOneKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstOneKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextOneKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstOneKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextOneKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_InsertTwoKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize, Buf: *mut i8);
    fn Star_SRPBasic_FindTwoKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize) -> *mut i8;
    fn Star_SRPBasic_DelTwoKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstTwoKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextTwoKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstTwoKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextTwoKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstTwoKey_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextTwoKey_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstTwoKeyA_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextTwoKeyA_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_InsertThreeKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize, ThirdKey: usize, Buf: *mut i8);
    fn Star_SRPBasic_FindThreeKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize, ThirdKey: usize) -> *mut i8;
    fn Star_SRPBasic_DelThreeKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize, ThirdKey: usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstThreeKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextThreeKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstThreeKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextThreeKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstThreeKey_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextThreeKey_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstThreeKeyA_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextThreeKeyA_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstThreeKey_S(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextThreeKey_S(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstThreeKeyA_S(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextThreeKeyA_S(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_InsertIDKey(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID, Buf: *mut i8);
    fn Star_SRPBasic_FindIDKey(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPBasic_DelIDKey(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPBasic_QueryFirstIDKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPBasic_QueryNextIDKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPBasic_QueryFirstIDKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPBasic_QueryNextIDKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPBasic_InsertIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID, ExKey: usize, Buf: *mut i8);
    fn Star_SRPBasic_FindIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID, ExKey: usize) -> *mut i8;
    fn Star_SRPBasic_DelIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID, ExKey: usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstIDKeyEx_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextIDKeyEx_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryFirstIDKeyExA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_QueryNextIDKeyExA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPBasic_GetKeyNumber(c_this: *const c_void, IndexContext: *const c_void) -> i32;
    fn Star_SRPBasic_DelAllKey(c_this: *const c_void, IndexContext: *const c_void);
    fn Star_SRPBasic_DestoryIndex(c_this: *const c_void, IndexContext: *const c_void);
    fn Star_SRPBasic_GetHashValue(c_this: *const c_void, Key: *mut c_void, Length: u32, InitValue: u32) -> u32;
    fn Star_SRPBasic_CreateMemory_Nor(c_this: *const c_void, ItemSize: i32) -> *mut c_void;
    fn Star_SRPBasic_CreateMemory_Dbg(c_this: *const c_void, ItemSize: i32, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPBasic_GetMemoryPtr_Nor(c_this: *const c_void, MemoryContext: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetMemoryPtr_Dbg(c_this: *const c_void, MemoryContext: *const c_void, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPBasic_QueryFirstMemoryPtr(c_this: *const c_void, MemoryContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_void;
    fn Star_SRPBasic_QueryNextMemoryPtr(c_this: *const c_void, MemoryContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_void;
    fn Star_SRPBasic_FreeMemoryPtr(c_this: *const c_void, MemoryContext: *const c_void, Ptr: *mut c_void);
    fn Star_SRPBasic_ClearMemory(c_this: *const c_void, MemoryContext: *const c_void);
    fn Star_SRPBasic_DestoryMemory(c_this: *const c_void, MemoryContext: *const c_void);
    fn Star_SRPBasic_Malloc_Nor(c_this: *const c_void, MemorySize: i32) -> *mut c_void;
    fn Star_SRPBasic_Malloc_Dbg(c_this: *const c_void, MemorySize: i32, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPBasic_Free(c_this: *const c_void, MemoryPtr: *mut c_void);
    fn Star_SRPBasic_GetMemoryUsed(c_this: *const c_void, KernelAllocSize: *mut usize, DataAllocSize: *mut usize, AppAllocSize: *mut usize, ScriptMemoryUsed: *mut usize);
    fn Star_SRPBasic_GetMachineID(c_this: *const c_void, Machine: *mut c_void) -> u32;
    fn Star_SRPBasic_FindMachine(c_this: *const c_void, MachineID: u32) -> *mut c_void;
    fn Star_SRPBasic_MD5ToUuid(c_this: *const c_void, String: *const i8, Uuid: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPBasic_UuidToMD5(c_this: *const c_void, Uuid: *mut VS_UUID) -> *mut i8;
    fn Star_SRPBasic_StringToUtf8(c_this: *const c_void, String: *const i8) -> *mut i8;
    fn Star_SRPBasic_Utf8ToString(c_this: *const c_void, String: *const i8) -> *mut i8;
    fn Star_SRPBasic_ProgramRestart(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_HttpDownLoad(c_this: *const c_void, AttachObjectID: *mut VS_UUID, ServerUrl: *const c_char, ClientPath: *const c_char, FileName: *const c_char, CallBackProc: *const c_void, ObjectID: *mut VS_UUID, Para: usize, SaveFileFlag: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_HttpDownLoadAbort(c_this: *const c_void);
    fn Star_SRPBasic_RegWebDownFunction(c_this: *const c_void, CallBackProc: *const c_void, Para: usize);
    fn Star_SRPBasic_UnRegWebDownFunction(c_this: *const c_void, CallBackProc: *const c_void, Para: usize);
    fn Star_SRPBasic_WebDownPrint(c_this: *const c_void, uMes: u32, FileName: *const c_char, MaxLength: u64, CurLength: u64);
    fn Star_SRPBasic_CanSetStaticData(c_this: *const c_void, Object: *mut c_void, DataSize: u32) -> VS_BOOL;
    fn Star_SRPBasic_SetStaticData(c_this: *const c_void, ObjectID: *mut VS_UUID, UniqueDataUnitID: u32, DataSize: u32, DataBuf: *mut i8, RetDataVersion: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPBasic_GetEnvDependCheckInfo(c_this: *const c_void, ServiceName: *const c_char, Size: *mut u32) -> *mut c_void;
    fn Star_SRPBasic_SetEnvDependCheckInfo(c_this: *const c_void, Size: u32, Buf: *mut c_void);
    fn Star_SRPBasic_GetEnvStartType(c_this: *const c_void) -> u8;
    fn Star_SRPBasic_GetEnvPara(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_SetEnvPara(c_this: *const c_void, Para: *mut c_void);
    fn Star_SRPBasic_SetEnvInputPara(c_this: *const c_void, Para: *mut c_void);
    fn Star_SRPBasic_GetEnvInputPara(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetEnvParentUrl(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPBasic_SetEnvCurrentUrl(c_this: *const c_void, Url: *const c_char);
    fn Star_SRPBasic_RedirectToUrlRequest(c_this: *const c_void, Url: *const c_char, ParaPkg: *mut c_void, CallBackProc: *const c_void, Para: usize, WorkDirectory: *const c_char, ChildTermScript: *const c_char) -> i32;
    fn Star_SRPBasic_RedirectToUrlFail(c_this: *const c_void, Url: *const c_char);
    fn Star_SRPBasic_SetRedirectToUrlInfo(c_this: *const c_void, ParaPkg: *mut c_void);
    fn Star_SRPBasic_RedirectToUrlAbort(c_this: *const c_void);
    fn Star_SRPBasic_OpenLuaEdit(c_this: *const c_void, Module: *const c_char, Config: u32, CloseEnable: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_LuaEditDisp(c_this: *const c_void, Info: *const c_char);
    fn Star_SRPBasic_CloseLuaEdit(c_this: *const c_void);
    fn Star_SRPBasic_LuaEditHelp(c_this: *const c_void, Type: i32, HelpInfo: *const c_char);
    fn Star_SRPBasic_RegDllCallBack(c_this: *const c_void, MsgCallBackProc: *const c_void, MsgCallBackPara: usize);
    fn Star_SRPBasic_UnRegDllCallBack(c_this: *const c_void, MsgCallBackProc: *const c_void, MsgCallBackPara: usize);
    fn Star_SRPBasic_CreateUuid(c_this: *const c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPBasic_GetSRPTempPath(c_this: *const c_void, BufSize: u32, Buf: *mut c_char);
    fn Star_SRPBasic_Compress(c_this: *const c_void, dest: *mut u8, destLen: *mut u32, source: *mut u8, sourceLen: u32) -> VS_BOOL;
    fn Star_SRPBasic_UnCompress(c_this: *const c_void, dest: *mut u8, destLen: *mut u32, source: *mut u8, sourceLen: u32) -> VS_BOOL;
    fn Star_SRPBasic_CreateMemoryFile(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetEnvMemoryFile(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_SetEnvMemoryFile(c_this: *const c_void, MemoryFile: *mut c_void);
    fn Star_SRPBasic_GetUrl(c_this: *const c_void, Buf: *mut c_char, BufSize: i32);
    fn Star_SRPBasic_ToAbsoluteUrl(c_this: *const c_void, InputUrl: *const c_char, OutputUrl: *mut c_char, OutputUrlBufSize: i32) -> VS_BOOL;
    fn Star_SRPBasic_RunFromUrl(c_this: *const c_void, Url: *const c_char, RestartFlag: i8, WaitFlag: VS_BOOL) -> i32;
    fn Star_SRPBasic_RunFromBuf(c_this: *const c_void, Buf: *mut c_char, BufSize: u32, RestartFlag: i8, WaitFlag: VS_BOOL) -> i32;
    fn Star_SRPBasic_IsLoadServiceBusy(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_IsLoadServiceIdle(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_SetDepend(c_this: *const c_void, ServiceName: *const c_char, DefaultUrlFlag: VS_BOOL);
    fn Star_SRPBasic_GetServicePathByName(c_this: *const c_void, ServiceName: *const c_char, ServicePath: *mut c_char, ServicePathSize: u32) -> VS_BOOL;
    fn Star_SRPBasic_InsertSearchPath(c_this: *const c_void, SearchPath: *const c_char);
    fn Star_SRPBasic_ClearSearchPath(c_this: *const c_void);
    fn Star_SRPBasic_FirstSearchPath(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_char;
    fn Star_SRPBasic_NextSearchPath(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_char;
    fn Star_SRPBasic_GetSaveFile(c_this: *const c_void, Caption: *const c_char, Filter: *const c_char, FilterIndex: u32, DefExt: *const c_char, FileNameBuf: *mut c_char, FileNameBufSize: i32) -> VS_BOOL;
    fn Star_SRPBasic_GetOpenFile(c_this: *const c_void, Caption: *const c_char, Filter: *const c_char, FilterIndex: u32, DefExt: *const c_char, FileNameBuf: *mut c_char, FileNameBufSize: i32) -> VS_BOOL;
    fn Star_SRPBasic_SaveServiceStaticData(c_this: *const c_void, ServiceID: *mut VS_UUID);
    fn Star_SRPBasic_GetStaticVersion(c_this: *const c_void, DataSize: u32, DataBuf: *mut i8, RetDataVersion: *mut VS_UUID);
    fn Star_SRPBasic_GetSysDocClass(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_FirstDoc(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, DocName: *mut [*mut c_char;1]) -> *mut c_void;
    fn Star_SRPBasic_NextDoc(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, DocName: *mut [*mut c_char;1]) -> *mut c_void;
    fn Star_SRPBasic_RegisterDoc(c_this: *const c_void, DocObject: *mut c_void, DocName: *const c_char);
    fn Star_SRPBasic_UnRegisterDoc(c_this: *const c_void, DocObject: *mut c_void);
    fn Star_SRPBasic_ProcessSysDocEvent(c_this: *const c_void, DocObjectID: *mut VS_UUID, EventID: *mut VS_UUID, RequestParam: *mut VS_EVENTPARAM_RUNPARAM) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPBasic_RegDocEventFunction(c_this: *const c_void, DocObjectID: *mut VS_UUID, EventID: *mut VS_UUID, FuncAddr: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPBasic_UnRegDocEventFunction(c_this: *const c_void, DocObjectID: *mut VS_UUID, EventID: *mut VS_UUID, FuncAddr: *mut c_void, Para: usize);
    fn Star_SRPBasic_ToClipBoard(c_this: *const c_void, Info: *const c_char);
    fn Star_SRPBasic_FromClipBoard(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPBasic_IsWindowlessSite(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_IsWindowlessTransparent(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_RegWindowlessSiteCallBack(c_this: *const c_void, CallBackInfo: *mut VSWindowlessSiteCallBackInfo, ObjectID: *mut VS_UUID, Para: usize);
    fn Star_SRPBasic_UnRegWindowlessSiteCallBack(c_this: *const c_void, CallBackInfo: *mut VSWindowlessSiteCallBackInfo, ObjectID: *mut VS_UUID, Para: usize);
    fn Star_SRPBasic_Windowless_Draw(c_this: *const c_void, hDC: *mut c_void, rcBounds: *mut c_void, rcInvalid: *mut c_void);
    fn Star_SRPBasic_Windowless_Message(c_this: *const c_void, uMes: u32, wParam: usize, LParam: usize, Result: *mut usize) -> VS_BOOL;
/*  fn Star_SRPBasic_Windowless_GetDropTarget() -> VS_BOOL; */
    fn Star_SRPBasic_Windowless_Redraw(c_this: *const c_void, fErase: VS_BOOL);
/*  fn Star_SRPBasic_Windowless_GetDC(); */
    fn Star_SRPBasic_Windowless_ReleaseDC(c_this: *const c_void, hDC: *mut c_void);
    fn Star_SRPBasic_SetClientPort(c_this: *const c_void, ClientInterface: *const c_char, ClientPortNumber: u16) -> VS_BOOL;
    fn Star_SRPBasic_SetDebugPort(c_this: *const c_void, DebugInterface: *const c_char, DebugPortNumber: u16) -> VS_BOOL;
    fn Star_SRPBasic_SetTelnetPort(c_this: *const c_void, TelnetPortNumber: u16) -> VS_BOOL;
    fn Star_SRPBasic_SetOutputPort(c_this: *const c_void, OutputHost: *const c_char, OutputPortNumber: u16) -> VS_BOOL;
    fn Star_SRPBasic_SetWebServerPort(c_this: *const c_void, WebServerHost: *const c_char, WebServerPortNumber: u16, ConnectionNumber: i32, PostSize: u32) -> VS_BOOL;
    fn Star_SRPBasic_GetVSObjectID(c_this: *const c_void, Which: i32) -> *mut VS_UUID;
    fn Star_SRPBasic_GetSRPControlInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetFunctionParaInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetParaPkgInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetSRPLockInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetSXMLInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetSRPBinBufInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetCommInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetFileDiskInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_GetSRPConfigPath(c_this: *const c_void, BufSize: u32, Buf: *mut c_char);
    fn Star_SRPBasic_RegTempFile(c_this: *const c_void, TempFileName: *const c_char, OriFileName: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_GetRegTempFile(c_this: *const c_void, OriFileName: *const c_char, Buf: *mut c_char, BufSize: i32) -> *mut c_char;
    fn Star_SRPBasic_UnRegTempFile(c_this: *const c_void, TempFileName: *const c_char);
    fn Star_SRPBasic_GetConfigResult(c_this: *const c_void, DebugCfgResult: *mut VS_BOOL, DirectClientCfgResult: *mut VS_BOOL, TelnetCfgResult: *mut VS_BOOL, WebServerCfgResult: *mut VS_BOOL);
    fn Star_SRPBasic_GetConfigEnvTag(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPBasic_RegDispatchCallBack(c_this: *const c_void, CallBack: *const c_void, Para: usize);
    fn Star_SRPBasic_UnRegDispatchCallBack(c_this: *const c_void, CallBack: *const c_void, Para: usize);
    fn Star_SRPBasic_Dup(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPBasic_QueryInterface(c_this: *const c_void, InterfaceID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPBasic_LockLuaTable(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_UnLockLuaTable(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_GetIDEx(c_this: *const c_void, Object: *mut c_void) -> *mut VS_UUID;
    fn Star_SRPBasic_IsRootService(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBasic_AddRef(c_this: *const c_void);
    fn Star_SRPBasic_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPBasic_GetConfig(c_this: *const c_void, XmlInterface: *mut c_void);
    fn Star_SRPBasic_GetConfigHost(c_this: *const c_void, Buf: *mut c_char, BufSize: i32);
    fn Star_SRPBasic_LuaToLString(c_this: *const c_void, index: i32, len: *mut u32) -> *mut c_char;
    fn Star_SRPBasic_LuaIsInt(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_ImportServiceFromXmlBuf(c_this: *const c_void, Buf: *const c_char, LoadRunModule: VS_BOOL) -> VS_BOOL;
    fn Star_SRPBasic_DoFileEx(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL, ModuleName: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_PostDoFileEx(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL, ModuleName: *const c_char) -> VS_BOOL;
    fn Star_SRPBasic_AddRefEx(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPBasic_DelRefEx(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPBasic_InitRaw(c_this: *const c_void, ScriptInterface: *const c_char, SRPInterface: *mut c_void) -> VS_BOOL;
    fn Star_SRPBasic_LoadRawModule(c_this: *const c_void, ScriptInterface: *const c_char, ModuleName: *const c_char, FileOrString: *const c_char, IsString: VS_BOOL, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPBasic_LoadRawModuleEx(c_this: *const c_void, ScriptInterface: *const c_char, ModuleName: *const c_char, Object: *mut c_void, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPBasic_DefScriptRawType(c_this: *const c_void, ScriptInterface: *const c_char, ModuleName: *const c_char, FileOrString: *const c_char, IsString: VS_BOOL, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPBasic_GetRawContextBuf(c_this: *const c_void, Object: *mut c_void, ScriptInterface: *const c_char) -> *mut c_void;
    fn Star_SRPBasic_NewScriptRawTypeGroup(c_this: *const c_void) -> i32;
    fn Star_SRPBasic_GetScriptRawTypeGroup(c_this: *const c_void, ScriptRawType: i32) -> i32;
    fn Star_SRPBasic_RegScriptRawType(c_this: *const c_void, ScriptInterface: *const c_char, TypeGroupName: *const c_char, GroupIndex: i32, TypeName: *const c_char) -> i32;
    fn Star_SRPBasic_GetScriptRawType(c_this: *const c_void, ScriptInterface: *const c_char, TypeGroupName: *const c_char, TypeName: *const c_char) -> i32;
    fn Star_SRPBasic_GetScriptRawTypeEx(c_this: *const c_void, RawType: i32, ScriptInterface: *mut [*mut c_char;1], TypeGroupName: *mut [*mut c_char;1]) -> *mut c_char;
    fn Star_SRPBasic_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPBasic_ReleaseOwnerEx(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPBasic_ReleaseOwnerExForScript(c_this: *const c_void, ScriptInterface: *const c_char, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPBasic_CaptureOwnerExForScript(c_this: *const c_void, ScriptInterface: *const c_char, Object: *mut c_void);
    fn Star_SRPBasic_GetRefEx(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPBasic_GetRefInfo(c_this: *const c_void, Object: *mut c_void) -> *mut c_char;
    fn Star_SRPBasic_LogObjectFreeByUnLock(c_this: *const c_void, Flag: VS_BOOL);
    fn Star_SRPBasic_SUnLockGC(c_this: *const c_void);
    fn Star_SRPBasic_GetLastError(c_this: *const c_void) -> i32;
    fn Star_SRPBasic_GetLastErrorInfo(c_this: *const c_void, LineIndex: *mut u32, SourceName: *mut [*mut c_char;1]) -> *mut c_char;
    fn Star_SRPBasic_GetCorePath(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPBasic_GetUserPath(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPBasic_GetLocalIP(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPBasic_GetLocalIPEx(c_this: *const c_void, SockAddr: *mut VSSOCKADDR_IN, ItemNumber: i32) -> i32;
    fn Star_SRPBasic_LuaPushInt64(c_this: *const c_void, Value: i64);
    fn Star_SRPBasic_LuaToInt64(c_this: *const c_void, Index: i32) -> i64;
    fn Star_SRPBasic_LuaIsInt64(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_LuaPushUWord(c_this: *const c_void, Value: usize);
    fn Star_SRPBasic_LuaToUWord(c_this: *const c_void, Index: i32) -> usize;
    fn Star_SRPBasic_LuaIsUWord(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPBasic_GetObjectNum(c_this: *const c_void) -> u32;
    fn Star_SRPBinBuf_Release(c_this: *const c_void);
    fn Star_SRPBinBuf_Init(c_this: *const c_void, BufSize: u32);
    fn Star_SRPBinBuf_GetSize(c_this: *const c_void) -> u32;
    fn Star_SRPBinBuf_GetOffset(c_this: *const c_void) -> u32;
    fn Star_SRPBinBuf_GetBuf(c_this: *const c_void) -> *mut i8;
    fn Star_SRPBinBuf_Clear(c_this: *const c_void);
    fn Star_SRPBinBuf_ClearEx(c_this: *const c_void, Offset: u32, Length: u32);
    fn Star_SRPBinBuf_Set(c_this: *const c_void, Offset: u32, Length: u32, Buf: *const i8) -> VS_BOOL;
    fn Star_SRPBinBuf_Get(c_this: *const c_void, Offset: u32, Length: u32, Buf: *mut i8) -> VS_BOOL;
    fn Star_SRPBinBuf_GetBufPtr(c_this: *const c_void, Offset: u32) -> *mut i8;
    fn Star_SRPBinBuf_GetBufPtrEx(c_this: *const c_void, Offset: u32, Length: u32) -> *mut i8;
    fn Star_SRPBinBuf_SetOffset(c_this: *const c_void, Offset: u32) -> VS_BOOL;
    fn Star_SRPBinBuf_Fill(c_this: *const c_void, Offset: u32, Length: u32, Value: u8) -> VS_BOOL;
    fn Star_SRPBinBuf_Expand(c_this: *const c_void, NewBufSize: i32) -> VS_BOOL;
    fn Star_SRPBinBuf_PackObject(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPBinBuf_UnPackObject(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPBinBuf_ToUTF8(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBinBuf_ToAnsi(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBinBuf_Dup(c_this: *const c_void) -> *mut c_void;
/*  fn Star_SRPBinBuf_Print(); */
/*  fn Star_SRPBinBuf_PrintVar(); */
    fn Star_SRPBinBuf_Insert(c_this: *const c_void, Offset: u32, Length: u32, Buf: *mut i8);
    fn Star_SRPBinBuf_FindStr(c_this: *const c_void, Offset: u32, Str: *const c_char) -> i32;
    fn Star_SRPBinBuf_FindStri(c_this: *const c_void, Offset: u32, Str: *const c_char) -> i32;
    fn Star_SRPBinBuf_SetLightBuf(c_this: *const c_void, Length: u32, Buf: *mut i8) -> VS_BOOL;
    fn Star_SRPBinBuf_IsLightBuf(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPBinBuf_AddRef(c_this: *const c_void);
    fn Star_SRPBinBuf_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPBinBuf_AnsiToUnicode(c_this: *const c_void, Code: *const c_char, BytesPerChar: i32) -> VS_BOOL;
    fn Star_SRPBinBuf_UnicodeToAnsi(c_this: *const c_void, Code: *const c_char, BytesPerChar: i32) -> VS_BOOL;
    fn Star_SRPBinBuf_WriteFromMemoryFile(c_this: *const c_void, SRPInterface: *mut c_void, Offset: u32, FileName: *const c_char) -> u32;
    fn Star_SRPBinBuf_GetMD5(c_this: *const c_void) -> *mut i8;
    fn Star_SRPBinBuf_GetHashValue(c_this: *const c_void) -> u32;
    fn Star_SRPBinBuf_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPBinBuf_FromRaw(c_this: *const c_void, In_FromBytesArray: VS_BOOL);
    fn Star_SRPBinBuf_IsFromRaw(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPParaPkg_Release(c_this: *const c_void);
    fn Star_SRPParaPkg_GetNumber(c_this: *const c_void) -> i32;
    fn Star_SRPParaPkg_Clear(c_this: *const c_void);
    fn Star_SRPParaPkg_InsertInt(c_this: *const c_void, Index: i32, Para: i32) -> VS_BOOL;
    fn Star_SRPParaPkg_InsertFloat(c_this: *const c_void, Index: i32, Para: f64) -> VS_BOOL;
    fn Star_SRPParaPkg_InsertStr(c_this: *const c_void, Index: i32, Para: *const c_char) -> VS_BOOL;
    fn Star_SRPParaPkg_InsertBin(c_this: *const c_void, Index: i32, Para: *mut i8, Size: i32) -> VS_BOOL;
    fn Star_SRPParaPkg_InsertTime(c_this: *const c_void, Index: i32, hTime: *mut VS_TIME) -> VS_BOOL;
    fn Star_SRPParaPkg_InsertEmpty(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPParaPkg_ExChange(c_this: *const c_void, DesIndex: i32, SrcIndex: i32) -> VS_BOOL;
    fn Star_SRPParaPkg_Del(c_this: *const c_void, Index: i32);
    fn Star_SRPParaPkg_GetType(c_this: *const c_void, Index: i32) -> i32;
    fn Star_SRPParaPkg_GetInt(c_this: *const c_void, Index: i32) -> i32;
    fn Star_SRPParaPkg_GetFloat(c_this: *const c_void, Index: i32) -> f64;
    fn Star_SRPParaPkg_GetStr(c_this: *const c_void, Index: i32) -> *mut c_char;
    fn Star_SRPParaPkg_GetBin(c_this: *const c_void, Index: i32, Length: *mut i32) -> *mut i8;
    fn Star_SRPParaPkg_GetTime(c_this: *const c_void, Index: i32, hTime: *mut VS_TIME) -> VS_BOOL;
    fn Star_SRPParaPkg_Dup(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPParaPkg_AppendFrom(c_this: *const c_void, ParaPkg: *mut c_void) -> VS_BOOL;
    fn Star_SRPParaPkg_SetChangeFlag(c_this: *const c_void, Index: i32);
    fn Star_SRPParaPkg_ClearChangeFlag(c_this: *const c_void, Index: i32);
    fn Star_SRPParaPkg_SetChangeFlagEx(c_this: *const c_void);
    fn Star_SRPParaPkg_ClearChangeFlagEx(c_this: *const c_void);
    fn Star_SRPParaPkg_IsChangeFlag(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPParaPkg_IsChangeFlagEx(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPParaPkg_SaveChangeToBuf(c_this: *const c_void, CompressFlag: VS_BOOL, RetSize: *mut i32) -> *mut i8;
    fn Star_SRPParaPkg_SaveChangeToBufEx(c_this: *const c_void, CompressFlag: VS_BOOL, RetSize: *mut i32) -> *mut i8;
    fn Star_SRPParaPkg_LoadChangeFromBuf(c_this: *const c_void, BufSize: i32, Buf: *mut i8) -> VS_BOOL;
    fn Star_SRPParaPkg_FreeBuf(c_this: *const c_void, Buf: *mut i8);
    fn Star_SRPParaPkg_AddRef(c_this: *const c_void);
    fn Star_SRPParaPkg_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPParaPkg_InsertBool(c_this: *const c_void, Index: i32, Para: VS_BOOL) -> VS_BOOL;
    fn Star_SRPParaPkg_GetBool(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPParaPkg_InsertObject(c_this: *const c_void, Index: i32, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPParaPkg_GetObject(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPParaPkg_InsertParaPackage(c_this: *const c_void, Index: i32, ParaPkg: *mut c_void) -> VS_BOOL;
    fn Star_SRPParaPkg_GetParaPackage(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPParaPkg_GetMaxSize(c_this: *const c_void) -> u32;
    fn Star_SRPParaPkg_SetScriptRawType(c_this: *const c_void, RawType: i32);
    fn Star_SRPParaPkg_GetScriptRawType(c_this: *const c_void) -> i32;
    fn Star_SRPParaPkg_ReleaseOwner(c_this: *const c_void);
/*  fn Star_SRPParaPkg_Build() -> *mut c_void; */
    fn Star_SRPParaPkg_AttachArrayObject(c_this: *const c_void, Object: *mut c_void, VarName: *const c_char, StartIndexOfArray: i32);
/*  fn Star_SRPParaPkg_BuildVar() -> *mut c_void; */
    fn Star_SRPParaPkg_AsDict(c_this: *const c_void, IsDict: VS_BOOL) -> *mut c_void;
    fn Star_SRPParaPkg_IsDict(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPParaPkg_FindDict(c_this: *const c_void, Key: *const c_char) -> i32;
    fn Star_SRPParaPkg_FindDictEx(c_this: *const c_void, Key: i32) -> i32;
    fn Star_SRPParaPkg_FromJSon(c_this: *const c_void, Buf: *const c_char) -> VS_BOOL;
    fn Star_SRPParaPkg_ToJSon(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPParaPkg_InsertInt64(c_this: *const c_void, Index: i32, Para: i64) -> VS_BOOL;
    fn Star_SRPParaPkg_GetInt64(c_this: *const c_void, Index: i32) -> i64;
    fn Star_SRPParaPkg_InsertStrEx(c_this: *const c_void, Index: i32, Para: *const c_char, ParaLength: u32) -> VS_BOOL;
    fn Star_SRPParaPkg_GetStrEx(c_this: *const c_void, Index: i32, ParaLength: *mut u32) -> *mut c_char;
    fn Star_SRPParaPkg_InsertBinEx(c_this: *const c_void, Index: i32, Para: *mut i8, Size: i32, FromBytesArray: VS_BOOL) -> VS_BOOL;
    fn Star_SRPParaPkg_GetBinEx(c_this: *const c_void, Index: i32, Length: *mut i32, FromBytesArray: *mut VS_BOOL) -> *mut i8;
    fn Star_SRPI_Release(c_this: *const c_void);
/*  fn Star_SRPI_GetObjectRegisterInfo() -> i32; */
/*  fn Star_SRPI_RegisterObjectDependency(); */
/*  fn Star_SRPI_RegisterObjectFunction(); */
/*  fn Star_SRPI_RegisterObjectSysEvent(); */
/*  fn Star_SRPI_RegisterObjectSysEditEvent(); */
/*  fn Star_SRPI_RegisterObjectInEvent(); */
/*  fn Star_SRPI_RegisterQueryObjectInfo(); */
/*  fn Star_SRPI_RegisterDynamicModule() -> i32; */
    fn Star_SRPI_GetOsType(c_this: *const c_void) -> u32;
    fn Star_SRPI_StringToUuid(c_this: *const c_void, String: *const i8, Uuid: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_UuidToString(c_this: *const c_void, Uuid: *mut VS_UUID) -> *mut i8;
    fn Star_SRPI_GetParent(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetIndex(c_this: *const c_void, Object: *mut c_void) -> u8;
    fn Star_SRPI_GetOrder(c_this: *const c_void, Object: *mut c_void) -> u16;
    fn Star_SRPI_GetClass(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetClassID(c_this: *const c_void, Object: *mut c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_GetID(c_this: *const c_void, Object: *mut c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_GetObject(c_this: *const c_void, ObjectID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_GetObjectEx(c_this: *const c_void, ParentObject: *mut c_void, Name: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetObjectEx2(c_this: *const c_void, ServiceName: *const c_char, Name: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetSRPObject(c_this: *const c_void, ObjectID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_GetSRPObjectEx(c_this: *const c_void, ParentObject: *mut c_void, Name: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetPrevEx(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetNextEx(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_QueryFirst(c_this: *const c_void, VSObject: *mut c_void) -> *mut c_void;
    fn Star_SRPI_QueryFirstChild(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8) -> *mut c_void;
    fn Star_SRPI_QueryNext(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_QueryPrev(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_QueryFirstEx(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_void;
    fn Star_SRPI_QueryNextEx(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_void;
    fn Star_SRPI_IsObject(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_QueryFirstActiveChild(c_this: *const c_void, Object: *mut c_void, Context: *mut usize) -> *mut c_void;
    fn Star_SRPI_QueryNextActiveChild(c_this: *const c_void, Context: *mut usize) -> *mut c_void;
    fn Star_SRPI_IsObjectInActiveSet(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_GetWebServiceFlag(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_SetWebServiceFlag(c_this: *const c_void, Object: *mut c_void, WebServiceFlag: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_QueryFirstInst(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ClassObject: *mut c_void) -> *mut c_void;
    fn Star_SRPI_QueryNextInst(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ClassObject: *mut c_void) -> *mut c_void;
    fn Star_SRPI_QueryFirstInstEx(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ObjectClassID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_QueryNextInstEx(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ObjectClassID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_QueryInstClose(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD);
    fn Star_SRPI_GetName(c_this: *const c_void, Object: *mut c_void) -> *mut c_char;
    fn Star_SRPI_SetName(c_this: *const c_void, Object: *mut c_void, Name: *const c_char);
    fn Star_SRPI_IsInst(c_this: *const c_void, ObjectClassID: *mut VS_UUID, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_IsDirectInst(c_this: *const c_void, ObjectClassID: *mut VS_UUID, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_IsChild(c_this: *const c_void, ParentObject: *mut c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_GetObjectSysRootItemID(c_this: *const c_void, Object: *mut c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_GetObjectSysRootItem(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_IsThisService(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_IsActiveServiceObject(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_IsThisClient(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_GetClientID(c_this: *const c_void, Object: *mut c_void) -> u32;
    fn Star_SRPI_ExportObjectHeader(c_this: *const c_void, Object: *mut c_void, FileName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_SetSaveFlag(c_this: *const c_void, Object: *mut c_void, SaveFlag: u8);
    fn Star_SRPI_GetSaveFlag(c_this: *const c_void, Object: *mut c_void) -> u8;
    fn Star_SRPI_SaveToBuf(c_this: *const c_void, Object: *mut c_void, ObjectSize: *mut i32, Password: *const c_char, SaveFlag: u8, SaveNameValue: VS_BOOL) -> *mut i8;
    fn Star_SRPI_SaveToFile(c_this: *const c_void, Object: *mut c_void, FileName: *const c_char, Password: *const c_char, SaveFlag: u8, SaveNameValue: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LoadFromBuf(c_this: *const c_void, Object: *mut c_void, Buf: *const i8, BufSize: i32, Password: *const c_char, LoadAsLocal: VS_BOOL, LoadNameValue: VS_BOOL, UpdateFlag: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_DeferLoadFromBuf(c_this: *const c_void, Object: *mut c_void, Buf: *const i8, BufSize: i32, Password: *const c_char, LoadAsLocal: VS_BOOL, LoadNameValue: VS_BOOL, UpdateFlag: VS_BOOL);
    fn Star_SRPI_LoadFromFile(c_this: *const c_void, Object: *mut c_void, FileName: *const c_char, Password: *const c_char, LoadAsLocal: VS_BOOL, LoadNameValue: VS_BOOL, UpdateFlag: VS_BOOL, StaticDataUseFile: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_DeferLoadFromFile(c_this: *const c_void, Object: *mut c_void, FileName: *const c_char, Password: *const c_char, LoadAsLocal: VS_BOOL, LoadNameValue: VS_BOOL, UpdateFlag: VS_BOOL, StaticDataUseFile: VS_BOOL);
    fn Star_SRPI_ResetLoad(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_RegLoadMapAttrNameProc(c_this: *const c_void, LoadMapAttrNameProc: *const c_void);
    fn Star_SRPI_IsLocalControl(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_IsRemoteCreate(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_SetIDInParent(c_this: *const c_void, Object: *mut c_void, IDInParent: u16);
    fn Star_SRPI_GetIDInParent(c_this: *const c_void, Object: *mut c_void) -> u16;
    fn Star_SRPI_GetChildByID(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8, IDInParent: u16) -> *mut c_void;
    fn Star_SRPI_GetFunctionID(c_this: *const c_void, Object: *mut c_void, FuntionName: *const c_char, FunctionID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_GetInEventID(c_this: *const c_void, Object: *mut c_void, InEventName: *const c_char, InEventID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_GetOutEventID(c_this: *const c_void, Object: *mut c_void, OutEventName: *const c_char, OutEventID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_GetOutEventName(c_this: *const c_void, OutEventID: *mut VS_UUID) -> *mut c_char;
    fn Star_SRPI_GetFunction(c_this: *const c_void, FunctionID: *mut VS_UUID, GlobalFunctionFlag: *mut VS_BOOL) -> *mut c_void;
    fn Star_SRPI_GetFunctionEx(c_this: *const c_void, Object: *mut c_void, FunctionID: *mut VS_UUID, GlobalFunctionFlag: *mut VS_BOOL) -> *mut c_void;
    fn Star_SRPI_GetOriFunction(c_this: *const c_void, FunctionID: *mut VS_UUID, GlobalFunctionFlag: *mut VS_BOOL) -> *mut c_void;
    fn Star_SRPI_SetFunction(c_this: *const c_void, FunctionID: *mut VS_UUID, FuncAddress: *mut c_void);
    fn Star_SRPI_SetInEvent(c_this: *const c_void, InEventID: *mut VS_UUID, InEventAddress: *mut c_void);
    fn Star_SRPI_GetSysEvent(c_this: *const c_void, Object: *mut c_void, Para: *mut usize) -> *mut c_void;
    fn Star_SRPI_SetSysEvent(c_this: *const c_void, Object: *mut c_void, SysEventAddress: *mut c_void, Para: usize);
    fn Star_SRPI_SetSysEventMask(c_this: *const c_void, Object: *mut c_void, EventMask: u32, EventProc: *const c_void);
    fn Star_SRPI_GetSysEventMask(c_this: *const c_void, Object: *mut c_void, EventProc: *const c_void) -> u32;
    fn Star_SRPI_SetChildEventMask(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32, EventMask: u32);
    fn Star_SRPI_GetChildEventMask(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32) -> u32;
    fn Star_SRPI_CreateOVLFunction(c_this: *const c_void, Object: *mut c_void, OriginFunctionID: *mut VS_UUID, FuncAddress: *mut c_void, NewFunctionID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_CreateOVLInEvent(c_this: *const c_void, Object: *mut c_void, OutEventID: *mut VS_UUID, InEventAddress: *mut c_void, NewInEventID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_EventIDToDWORD(c_this: *const c_void, OutEventID: *mut VS_UUID) -> u32;
    fn Star_SRPI_EventNameToDWORD(c_this: *const c_void, OutEventID: *mut VS_UUID) -> u32;
    fn Star_SRPI_CreateNameScript(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, ScriptBuf: *const c_char) -> VS_BOOL;
    fn Star_SRPI_CreateNameScriptEx(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, FileName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_DeleteNameScript(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char);
    fn Star_SRPI_ChangeScriptName(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, NewScriptName: *const c_char);
    fn Star_SRPI_GetNameScript(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char) -> *mut c_char;
    fn Star_SRPI_QueryFirstNameScript(c_this: *const c_void, Object: *mut c_void, ScriptPtr: *mut [*mut c_char;1]) -> *mut c_char;
    fn Star_SRPI_QueryNextNameScript(c_this: *const c_void, ScriptPtr: *mut [*mut c_char;1]) -> *mut c_char;
    fn Star_SRPI_ExecNameScript(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, nArgs: i32, nOutArgs: i32) -> VS_BOOL;
    fn Star_SRPI_ExecNameScriptEx(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, nArgs: i32, nOutArgs: i32) -> VS_BOOL;
    fn Star_SRPI_ExecScript(c_this: *const c_void, Object: *mut c_void, FuncName: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, nArgs: i32, nOutArgs: i32) -> VS_BOOL;
    fn Star_SRPI_ExecScriptEx(c_this: *const c_void, Object: *mut c_void, FuncName: *const c_char, FileName: *const c_char, nArgs: i32, nOutArgs: i32) -> VS_BOOL;
    fn Star_SRPI_ForceReCompile(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char);
    fn Star_SRPI_RegCompileFunc(c_this: *const c_void, HookProc: *const c_void, Para: usize);
/*  fn Star_SRPI_RemoteCall(); */
/*  fn Star_SRPI_RemoteCallVar(); */
/*  fn Star_SRPI_RemoteCallEx(); */
/*  fn Star_SRPI_RemoteCallExVar(); */
    fn Star_SRPI_GetRemoteID(c_this: *const c_void, Object: *mut c_void) -> u32;
    fn Star_SRPI_GetRemoteCallID(c_this: *const c_void, Object: *mut c_void) -> u32;
    fn Star_SRPI_GetRemoteSourceTag(c_this: *const c_void, Object: *mut c_void) -> u16;
    fn Star_SRPI_GetRemoteAttach(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetRemoteCallName(c_this: *const c_void, Object: *mut c_void) -> *mut c_char;
    fn Star_SRPI_IsRemoteCall(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
/*  fn Star_SRPI_SRemoteCall() -> usize; */
/*  fn Star_SRPI_SRemoteCallVar() -> usize; */
/*  fn Star_SRPI_ARemoteCall() -> VS_BOOL; */
/*  fn Star_SRPI_ARemoteCallVar() -> VS_BOOL; */
    fn Star_SRPI_SetDeferRspFlag(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_SetRetCode(c_this: *const c_void, Object: *mut c_void, RemoteRetCode: u32);
    fn Star_SRPI_SetRemoteRspAttach(c_this: *const c_void, Object: *mut c_void, RemoteAttach: *mut c_void);
    fn Star_SRPI_RemoteCallRsp(c_this: *const c_void, Object: *mut c_void, ClientID: u32, RemoteCallID: u32, RemoteCallName: *const c_char, RemoteSourceTag: u16, RetCode: u32, RetType: u8, RetValue: usize, RemoteAttach: *mut c_void);
    fn Star_SRPI_FillSoapRspHeader(c_this: *const c_void, SXMLInterface: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_SetPrivateTag(c_this: *const c_void, ClientPrivateTag: u32);
    fn Star_SRPI_GetRemotePrivateTag(c_this: *const c_void, Object: *mut c_void) -> u32;
    fn Star_SRPI_GetLayer(c_this: *const c_void, Object: *mut c_void) -> u32;
    fn Star_SRPI_SetPrivateValue(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32, Index: u32, Value: usize);
    fn Star_SRPI_GetPrivateValue(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32, Index: u32, Value: *mut usize, DefaultValue: usize) -> VS_BOOL;
    fn Star_SRPI_MallocPrivateBuf(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32, Index: u32, BufSize: i32) -> *mut c_void;
    fn Star_SRPI_GetPrivateBuf(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32, Index: u32, BufSize: *mut i32) -> *mut c_void;
    fn Star_SRPI_FreePrivate(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32, Index: u32);
    fn Star_SRPI_SetNameIntValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: i32, LocalChange: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_GetNameIntValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: *mut i32, DefaultValue: i32) -> VS_BOOL;
    fn Star_SRPI_SetNameFloatValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: f64, LocalChange: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_GetNameFloatValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: *mut f64, DefaultValue: f64) -> VS_BOOL;
    fn Star_SRPI_SetNameBinValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: *mut i8, ValueSize: u16, LocalChange: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_GetNameBinValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, ValueSize: *mut u16) -> *mut i8;
    fn Star_SRPI_SetNameStrValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: *const c_char, LocalChange: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_GetNameStrValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, DefaultValue: *const c_char) -> *mut c_char;
    fn Star_SRPI_SetNameTimeValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: *mut VS_TIME, LocalChange: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_GetNameTimeValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: *mut VS_TIME, DefaultValue: *mut VS_TIME) -> VS_BOOL;
    fn Star_SRPI_FreeNameValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char);
    fn Star_SRPI_FreeAllNameValue(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_GetNameValueType(c_this: *const c_void, Object: *mut c_void, Name: *const c_char) -> u8;
    fn Star_SRPI_QueryFirstNameValue(c_this: *const c_void, Object: *mut c_void, Context: *mut usize, Type: *mut u8) -> *mut c_char;
    fn Star_SRPI_QueryNextNameValue(c_this: *const c_void, Object: *mut c_void, Context: *mut usize, Type: *mut u8) -> *mut c_char;
    fn Star_SRPI_RegNameValueChangeCallBack(c_this: *const c_void, Object: *mut c_void, ObjectNameValueChangeNotifyProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegNameValueChangeCallBack(c_this: *const c_void, Object: *mut c_void, ObjectNameValueChangeNotifyProc: *const c_void, Para: usize);
    fn Star_SRPI_GetSyncStatus(c_this: *const c_void, Object: *mut c_void) -> u8;
    fn Star_SRPI_GetSyncGroupStatus(c_this: *const c_void, SysRootItemID: *mut VS_UUID, GroupIndex: u32) -> u8;
    fn Star_SRPI_GetSyncGroupStatusEx(c_this: *const c_void, SysRootItemName: *mut c_char, GroupIndex: u32) -> u8;
    fn Star_SRPI_SetSyncGroup(c_this: *const c_void, Object: *mut c_void, GroupIndex: u32);
    fn Star_SRPI_GetSyncGroup(c_this: *const c_void, Object: *mut c_void, GroupIndex: *mut u32);
    fn Star_SRPI_GetActiveService(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetService(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetActiveServicePath(c_this: *const c_void, Buf: *mut i8, BufSize: i32) -> i32;
    fn Star_SRPI_GetActiveServiceName(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPI_GetServicePath(c_this: *const c_void, Buf: *mut i8, BufSize: i32) -> i32;
    fn Star_SRPI_GetServiceName(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPI_GetServiceInterval(c_this: *const c_void) -> i32;
    fn Star_SRPI_GetServiceID(c_this: *const c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_StartVSService(c_this: *const c_void, ServiceID: *mut VS_UUID);
    fn Star_SRPI_ExitVSService(c_this: *const c_void);
    fn Star_SRPI_SaveService(c_this: *const c_void, Path: *const c_char);
    fn Star_SRPI_IsServiceChange(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_IsServiceActive(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_GetServiceInfo(c_this: *const c_void, ServiceInfo: *mut VS_SERVICEINFO);
    fn Star_SRPI_QueryFirstDepend(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ServiceID: *mut VS_UUID, RetUuid: *mut VS_UUID, RetServiceName: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPI_QueryNextDepend(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, RetUuid: *mut VS_UUID, RetServiceName: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPI_IsOsSupport(c_this: *const c_void, ProgramRunType: u16, OsType: u32) -> VS_BOOL;
    fn Star_SRPI_GetFrameTicket(c_this: *const c_void) -> usize;
    fn Star_SRPI_ExportModule(c_this: *const c_void, XmlCfgFile: *const c_char, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPI_GetServiceGroupID(c_this: *const c_void) -> u32;
/*  fn Star_SRPI_Print(); */
/*  fn Star_SRPI_PrintVar(); */
/*  fn Star_SRPI_PrintLua(); */
/*  fn Star_SRPI_PrintLuaVar(); */
    fn Star_SRPI_SetPrintToLua(c_this: *const c_void, PrintFlag: VS_BOOL);
/*  fn Star_SRPI_MessageBox(); */
/*  fn Star_SRPI_MessageBoxVar(); */
    fn Star_SRPI_RegMessageBoxFunction(c_this: *const c_void, Object: *mut c_void, MessageBoxProc: *const c_void);
    fn Star_SRPI_UnRegMessageBoxFunction(c_this: *const c_void, Object: *mut c_void, MessageBoxProc: *const c_void);
/*  fn Star_SRPI_ProcessError(); */
/*  fn Star_SRPI_ProcessErrorVar(); */
/*  fn Star_SRPI_ProcessLuaError(); */
/*  fn Star_SRPI_ProcessLuaErrorVar(); */
    fn Star_SRPI_IsBeingTrace(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
/*  fn Star_SRPI_Trace(); */
/*  fn Star_SRPI_TraceVar(); */
    fn Star_SRPI_CaptureLuaDisp(c_this: *const c_void, DispProc: *const c_void, Para: usize);
    fn Star_SRPI_ReleaseLuaDisp(c_this: *const c_void, DispProc: *const c_void, Para: usize);
    fn Star_SRPI_InsertToSDT(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_DelFromSDT(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_QueryFirstFromSDT(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_void;
    fn Star_SRPI_QueryNextFromSDT(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_void;
    fn Star_SRPI_QueryFirstInstFromSDT(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ObjectClassID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_QueryNextInstFromSDT(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ObjectClassID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_MallocStaticObject(c_this: *const c_void, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_MallocStaticObjectEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_MallocGlobalObject(c_this: *const c_void, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void, ClientID: u32) -> *mut c_void;
    fn Star_SRPI_MallocGlobalObjectEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void, ClientID: u32) -> *mut c_void;
    fn Star_SRPI_MallocObject(c_this: *const c_void, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_MallocObjectEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_MallocObjectL(c_this: *const c_void, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_MallocObjectLEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_MallocClientObject(c_this: *const c_void, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void, ClientID: u32) -> *mut c_void;
    fn Star_SRPI_MallocClientObjectEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, AttachBufSize: i32, AttachBuf: *mut c_void, ClientID: u32) -> *mut c_void;
    fn Star_SRPI_WaitMalloc(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_GetOPPermission(c_this: *const c_void) -> u32;
    fn Star_SRPI_CopyObject(c_this: *const c_void, Object: *mut c_void, SrcObject: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_FreeObject(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_DeferFreeObject(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_IsObjectInFree(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_ChangeLocal(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8, NewValue: *const i8);
    fn Star_SRPI_ChangeObject(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8, NewValue: *const i8);
    fn Star_SRPI_ChangeParent(c_this: *const c_void, Object: *mut c_void, ParentObject: *mut c_void, AttributeIndex: u8);
    fn Star_SRPI_MarkChange(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8);
    fn Star_SRPI_RegBeforeChangeCallBack(c_this: *const c_void, Object: *mut c_void, ObjectBeforeChangeNotifyProc: *const c_void, Para: usize, ChildNotify: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_RegChangeCallBack(c_this: *const c_void, Object: *mut c_void, ObjectChangeNotifyProc: *const c_void, Para: usize, ChildNotify: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_RegReMallocCallBack(c_this: *const c_void, Object: *mut c_void, ObjectReMallocNotifyProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegBeforeChangeCallBack(c_this: *const c_void, Object: *mut c_void, ObjectBeforeChangeNotifyProc: *const c_void, Para: usize);
    fn Star_SRPI_UnRegChangeCallBack(c_this: *const c_void, Object: *mut c_void, ObjectChangeNotifyProc: *const c_void, Para: usize);
    fn Star_SRPI_UnRegReMallocCallBack(c_this: *const c_void, Object: *mut c_void, ObjectReMallocNotifyProc: *const c_void, Para: usize);
    fn Star_SRPI_RegObjectIDChangeNotify(c_this: *const c_void, ChangeNotifyProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegObjectIDChangeNotify(c_this: *const c_void, ChangeNotifyProc: *const c_void, Para: usize);
    fn Star_SRPI_RegObjectFreeNotify(c_this: *const c_void, FreeNotifyProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegObjectFreeNotify(c_this: *const c_void, FreeNotifyProc: *const c_void, Para: usize);
    fn Star_SRPI_DupVString(c_this: *const c_void, InVString: *mut VS_VSTRING, OutVString: *mut VS_VSTRING);
    fn Star_SRPI_GetVStringBufSize(c_this: *const c_void, VString: *mut VS_VSTRING) -> u32;
    fn Star_SRPI_ExpandVStringBufSize(c_this: *const c_void, VString: *mut VS_VSTRING, Size: u32);
    fn Star_SRPI_GetAllocType(c_this: *const c_void, Object: *mut c_void) -> u32;
    fn Star_SRPI_CreateUser(c_this: *const c_void, In_UserName: *const c_char, UserPass: *const c_char, ReadWriteOrExecute: u8) -> VS_BOOL;
    fn Star_SRPI_DeleteUser(c_this: *const c_void, In_UserName: *const c_char);
    fn Star_SRPI_QueryFirstUser(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ReadWriteOrExecute: *mut u8) -> *mut c_char;
    fn Star_SRPI_QueryNextUser(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ReadWriteOrExecute: *mut u8) -> *mut c_char;
    fn Star_SRPI_SetAppClass(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32, SRPObjectClass: *mut c_void);
    fn Star_SRPI_GetAppClass(c_this: *const c_void, Object: *mut c_void, ClassLayer: u32) -> *mut c_void;
    fn Star_SRPI_GetAttributeNumber(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_GetAttributeSize(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_GetAttributeName(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8) -> *mut c_char;
    fn Star_SRPI_GetAttributeInfo(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8, AttributeInfo: *mut VS_ATTRIBUTEINFO) -> VS_BOOL;
    fn Star_SRPI_GetAttributeInfoEx(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, AttributeInfo: *mut VS_ATTRIBUTEINFO) -> VS_BOOL;
    fn Star_SRPI_GetAttachAttributeNumber(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_GetAttachAttributeSize(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_GetAttachAttributeInfo(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8, AttributeInfo: *mut VS_ATTRIBUTEINFO) -> VS_BOOL;
    fn Star_SRPI_GetComboBoxItem(c_this: *const c_void, ComboBoxID: *mut u8, ComboBoxItemBuf: *mut VS_COMBOBOXITEM) -> VS_BOOL;
    fn Star_SRPI_GetFunctionNumber(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_GetFunctionInfo(c_this: *const c_void, Object: *mut c_void, FunctionIndex: i32, FunctionInfo: *mut VS_FUNCTIONINFO) -> VS_BOOL;
    fn Star_SRPI_GetOutEventNumber(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_GetOutEventInfo(c_this: *const c_void, Object: *mut c_void, OutEventIndex: i32, OutEventInfo: *mut VS_OUTEVENTINFO) -> VS_BOOL;
    fn Star_SRPI_CreateIndex_Nor(c_this: *const c_void, KeyNumber: i32, HashTableBits: u16) -> *mut c_void;
    fn Star_SRPI_CreateIndexCmp_Nor(c_this: *const c_void, KeyNumber: i32, HashTableBits: u16, CompareProc: *const c_void) -> *mut c_void;
    fn Star_SRPI_CreateIDIndex_Nor(c_this: *const c_void, HashTableBits: u16) -> *mut c_void;
    fn Star_SRPI_CreateIDIndexEx_Nor(c_this: *const c_void, HashTableBits: u16) -> *mut c_void;
    fn Star_SRPI_CreateIndex_Dbg(c_this: *const c_void, KeyNumber: i32, HashTableBits: u16, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPI_CreateIndexCmp_Dbg(c_this: *const c_void, KeyNumber: i32, HashTableBits: u16, CompareProc: *const c_void, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPI_CreateIDIndex_Dbg(c_this: *const c_void, HashTableBits: u16, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPI_CreateIDIndexEx_Dbg(c_this: *const c_void, HashTableBits: u16, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPI_InsertOneKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, Buf: *mut i8);
    fn Star_SRPI_FindOneKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize) -> *mut i8;
    fn Star_SRPI_DelOneKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize) -> *mut i8;
    fn Star_SRPI_QueryFirstOneKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextOneKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstOneKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextOneKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize) -> *mut i8;
    fn Star_SRPI_InsertTwoKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize, Buf: *mut i8);
    fn Star_SRPI_FindTwoKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize) -> *mut i8;
    fn Star_SRPI_DelTwoKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize) -> *mut i8;
    fn Star_SRPI_QueryFirstTwoKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextTwoKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstTwoKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextTwoKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstTwoKey_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextTwoKey_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstTwoKeyA_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextTwoKeyA_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize) -> *mut i8;
    fn Star_SRPI_InsertThreeKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize, ThirdKey: usize, Buf: *mut i8);
    fn Star_SRPI_FindThreeKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize, ThirdKey: usize) -> *mut i8;
    fn Star_SRPI_DelThreeKey(c_this: *const c_void, IndexContext: *const c_void, MainKey: usize, SecondKey: usize, ThirdKey: usize) -> *mut i8;
    fn Star_SRPI_QueryFirstThreeKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextThreeKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstThreeKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextThreeKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: *mut usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstThreeKey_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextThreeKey_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstThreeKeyA_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextThreeKeyA_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: *mut usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstThreeKey_S(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextThreeKey_S(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstThreeKeyA_S(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextThreeKeyA_S(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, MainKey: usize, SecondKey: usize, ThirdKey: *mut usize) -> *mut i8;
    fn Star_SRPI_InsertIDKey(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID, Buf: *mut i8);
    fn Star_SRPI_FindIDKey(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPI_DelIDKey(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPI_QueryFirstIDKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPI_QueryNextIDKey(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPI_QueryFirstIDKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPI_QueryNextIDKeyA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID) -> *mut i8;
    fn Star_SRPI_InsertIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID, ExKey: usize, Buf: *mut i8);
    fn Star_SRPI_FindIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID, ExKey: usize) -> *mut i8;
    fn Star_SRPI_DelIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, UuidKey: *mut VS_UUID, ExKey: usize) -> *mut i8;
    fn Star_SRPI_QueryFirstIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextIDKeyEx(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstIDKeyEx_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextIDKeyEx_F(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryFirstIDKeyExA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPI_QueryNextIDKeyExA(c_this: *const c_void, IndexContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD, UuidKey: *mut VS_UUID, ExKey: *mut usize) -> *mut i8;
    fn Star_SRPI_GetKeyNumber(c_this: *const c_void, IndexContext: *const c_void) -> i32;
    fn Star_SRPI_DelAllKey(c_this: *const c_void, IndexContext: *const c_void);
    fn Star_SRPI_DestoryIndex(c_this: *const c_void, IndexContext: *const c_void);
    fn Star_SRPI_GetHashValue(c_this: *const c_void, Key: *mut c_void, Length: u32, InitValue: u32) -> u32;
    fn Star_SRPI_CreateMemory_Nor(c_this: *const c_void, ItemSize: i32) -> *mut c_void;
    fn Star_SRPI_CreateMemory_Dbg(c_this: *const c_void, ItemSize: i32, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPI_GetMemoryPtr_Nor(c_this: *const c_void, MemoryContext: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetMemoryPtr_Dbg(c_this: *const c_void, MemoryContext: *const c_void, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPI_QueryFirstMemoryPtr(c_this: *const c_void, MemoryContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_void;
    fn Star_SRPI_QueryNextMemoryPtr(c_this: *const c_void, MemoryContext: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_void;
    fn Star_SRPI_FreeMemoryPtr(c_this: *const c_void, MemoryContext: *const c_void, Ptr: *mut c_void);
    fn Star_SRPI_ClearMemory(c_this: *const c_void, MemoryContext: *const c_void);
    fn Star_SRPI_DestoryMemory(c_this: *const c_void, MemoryContext: *const c_void);
    fn Star_SRPI_Malloc_Nor(c_this: *const c_void, MemorySize: i32) -> *mut c_void;
    fn Star_SRPI_Malloc_Dbg(c_this: *const c_void, MemorySize: i32, FileName: *const c_char, LineNumber: i32) -> *mut c_void;
    fn Star_SRPI_Free(c_this: *const c_void, MemoryPtr: *mut c_void);
    fn Star_SRPI_GetMemoryUsed(c_this: *const c_void, KernelAllocSize: *mut usize, DataAllocSize: *mut usize, AppAllocSize: *mut usize, ScriptMemoryUsed: *mut usize);
    fn Star_SRPI_InjectSysEventFunction(c_this: *const c_void, SysEventID: u32, ProcessFunctionProc: *const c_void);
    fn Star_SRPI_RejectSysEventFunction(c_this: *const c_void, SysEventID: u32, ProcessFunctionProc: *const c_void);
    fn Star_SRPI_GetResponseBuf(c_this: *const c_void) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPI_GetRequestBuf(c_this: *const c_void) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPI_GetSysEventID(c_this: *const c_void, EventParam: *mut VS_EVENTPARAM) -> u32;
    fn Star_SRPI_FreeResponseBuf(c_this: *const c_void, ResponseParam: *mut VS_EVENTPARAM_RUNPARAM);
    fn Star_SRPI_FreeRequestBuf(c_this: *const c_void, RequestParam: *mut VS_EVENTPARAM_RUNPARAM);
    fn Star_SRPI_AttachResponseBuf(c_this: *const c_void, EventParam: *mut VS_EVENTPARAM, ResponseParam: *mut VS_EVENTPARAM_RUNPARAM);
    fn Star_SRPI_ProcessEvent_Nor(c_this: *const c_void, EventID: *mut VS_UUID, SrcObject: *mut c_void, DesObject: *mut c_void, RequestParam: *mut VS_EVENTPARAM_RUNPARAM) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPI_ProcessEvent_Dbg(c_this: *const c_void, EventID: *mut VS_UUID, SrcObject: *mut c_void, DesObject: *mut c_void, RequestParam: *mut VS_EVENTPARAM_RUNPARAM) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPI_PostProcessEvent_Nor(c_this: *const c_void, EventID: *mut VS_UUID, SrcObject: *mut c_void, DesObject: *mut c_void, RequestParam: *mut VS_EVENTPARAM_RUNPARAM) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPI_PostProcessEvent_Dbg(c_this: *const c_void, EventID: *mut VS_UUID, SrcObject: *mut c_void, DesObject: *mut c_void, RequestParam: *mut VS_EVENTPARAM_RUNPARAM) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPI_ProcessParentEvent(c_this: *const c_void, EventParam: *mut VS_EVENTPARAM) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPI_SetEventAttachBuf(c_this: *const c_void, RunParam: *mut VS_EVENTPARAM_RUNPARAM);
    fn Star_SRPI_RegEventFunction(c_this: *const c_void, SrcObject: *mut c_void, EventID: *mut VS_UUID, Object: *mut c_void, FuncAddr: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegEventFunction(c_this: *const c_void, SrcObject: *mut c_void, EventID: *mut VS_UUID, Object: *mut c_void, FuncAddr: *mut c_void, Para: usize);
    fn Star_SRPI_RegSysEventFunction(c_this: *const c_void, Object: *mut c_void, SysEventID: u32, FuncAddr: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegSysEventFunction(c_this: *const c_void, Object: *mut c_void, SysEventID: u32, FuncAddr: *mut c_void, Para: usize);
    fn Star_SRPI_ActiveCmd(c_this: *const c_void, Object: *mut c_void, ActiveCmd: u8) -> VS_BOOL;
    fn Star_SRPI_GetActiveCmd(c_this: *const c_void, Object: *mut c_void) -> u8;
    fn Star_SRPI_ActiveClient(c_this: *const c_void, ClientID: u32, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_DeactiveClient(c_this: *const c_void, ClientID: u32, Object: *mut c_void);
    fn Star_SRPI_Active(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_Deactive(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_DeactiveAll(c_this: *const c_void);
    fn Star_SRPI_IsActive(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_QueryFirstActiveInst(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ObjectClassID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_QueryNextActiveInst(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, ObjectClassID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_SetClientObject(c_this: *const c_void, ClientID: u32, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_GetClientObject(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_CreateSysRootItem(c_this: *const c_void, SystemRootItemName: *const c_char, DependSysRootItem: *const c_char, SystemRootItemID: *mut VS_UUID, SystemRootItemIDEx: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_ActiveAllSysRootItem(c_this: *const c_void);
    fn Star_SRPI_ActiveSysRootItem(c_this: *const c_void, SystemRootItemName: *const c_char);
    fn Star_SRPI_DeactiveSysRootItem(c_this: *const c_void, SystemRootItemName: *const c_char);
    fn Star_SRPI_ActiveCSysRootItem(c_this: *const c_void, ClientID: u32, SystemRootItemName: *const c_char);
    fn Star_SRPI_DeactiveCSysRootItem(c_this: *const c_void, ClientID: u32, SystemRootItemName: *const c_char);
    fn Star_SRPI_GetSysRootItem(c_this: *const c_void, SystemRootItemName: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetSysRootItemEx(c_this: *const c_void, SystemRootItemID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_GetSysRootItemName(c_this: *const c_void, SystemRootItem: *mut c_void) -> *mut c_char;
    fn Star_SRPI_GetSysRootItemActiveSet(c_this: *const c_void, SystemRootItem: *mut c_void, ActiveSetPtr: *mut VS_ACTIVESETITEM);
    fn Star_SRPI_SetSysRootItemActiveSet(c_this: *const c_void, SystemRootItem: *mut c_void, ActiveSetItem: *mut VS_ACTIVESETITEM);
    fn Star_SRPI_SetCSysRootItemActiveSet(c_this: *const c_void, ClientID: u32, SystemRootItem: *mut c_void, ActiveSetItem: *mut VS_ACTIVESETITEM);
    fn Star_SRPI_QueryFirstSysRootItem(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPI_QueryNextSysRootItem(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPI_QueryFirstGroupObject(c_this: *const c_void, SystemRootItem: *mut c_void, GroupIndex: u32, QueryGroupContext: *mut usize) -> *mut c_void;
    fn Star_SRPI_QueryNextGroupObject(c_this: *const c_void, QueryGroupContext: *mut usize) -> *mut c_void;
    fn Star_SRPI_QueryFirstSysRootItemChild(c_this: *const c_void, SystemRootItem: *mut c_void) -> *mut c_void;
    fn Star_SRPI_RegClientSysRootItemToSyncFunc(c_this: *const c_void, SystemRootItem: *mut c_void, FuncPtr: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegClientSysRootItemToSyncFunc(c_this: *const c_void, SystemRootItem: *mut c_void, FuncPtr: *const c_void, Para: usize);
    fn Star_SRPI_InitEdit(c_this: *const c_void, ClassObject: *mut c_void, Object: *mut c_void);
    fn Star_SRPI_TermEdit(c_this: *const c_void, ClassObject: *mut c_void, Object: *mut c_void);
    fn Star_SRPI_GetEditMode(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_SetEditMode(c_this: *const c_void, Object: *mut c_void, EditFlag: VS_BOOL);
    fn Star_SRPI_EditCommit(c_this: *const c_void);
    fn Star_SRPI_EditSelect(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_EditChange(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8, NewValue: *const i8);
    fn Star_SRPI_EditMarkChange(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8);
    fn Star_SRPI_EditSetSaveFlag(c_this: *const c_void, Object: *mut c_void, SaveFlag: u8);
    fn Star_SRPI_EditSetName(c_this: *const c_void, Object: *mut c_void, Name: *const c_char);
    fn Star_SRPI_FillAttachBuf(c_this: *const c_void, ObjectClassID: *mut VS_UUID, AttachBuf: *mut i8, AttachBufSize: i32);
    fn Star_SRPI_IsEditProcExist(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_TriggerEditProc(c_this: *const c_void, ClassObject: *mut c_void, Object: *mut c_void, AppCode: usize, AppCode1: usize);
    fn Star_SRPI_EditDelete(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_EditCreate(c_this: *const c_void, ObjectClassID: *mut VS_UUID, ParentObjectID: *mut VS_UUID, AttributeIndex: u8, OrderIndex: u32, ObjectSaveFlag: u8, AttachBufSize: i32, AttachBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_EditCreateEx(c_this: *const c_void, ObjectClassID: *mut VS_UUID, ObjectID: *mut VS_UUID, ParentObjectID: *mut VS_UUID, AttributeIndex: u8, OrderIndex: u32, ObjectSaveFlag: u8, AttachBufSize: i32, AttachBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_EditChangeParent(c_this: *const c_void, Object: *mut c_void, ParentObject: *mut c_void, AttributeIndex: u8);
    fn Star_SRPI_EditChangeSyncGroup(c_this: *const c_void, Object: *mut c_void, GroupIndex: u32);
    fn Star_SRPI_EditGetClassID(c_this: *const c_void, ObjectID: *mut VS_UUID, AttributeIndex: u8, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_EditGetInstID(c_this: *const c_void, ObjectClassID: *mut VS_UUID, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_EditSetWndStatus(c_this: *const c_void, Normal: VS_BOOL);
    fn Star_SRPI_EditUpdateObjectScript(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, ScriptBuf: *const c_char) -> VS_BOOL;
    fn Star_SRPI_EditUpdateObjectScriptEx(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, FileName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_RegClientOpFunction(c_this: *const c_void, ClientOperationCallBackProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegClientOpFunction(c_this: *const c_void, ClientOperationCallBackProc: *const c_void, Para: usize);
    fn Star_SRPI_Redirect(c_this: *const c_void, ClientID: u32, DesServerInterface: *const c_char, DesServerName: *const c_char, DesServerPortNumber: u16, ParaPkg: *mut c_void, RedirectCallBackProc: *const c_void, Para: usize);
    fn Star_SRPI_GetServiceMachine(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_DelMachine(c_this: *const c_void, Machine: *mut c_void);
    fn Star_SRPI_GetMachineAttachBuf(c_this: *const c_void, Machine: *mut c_void) -> *mut i8;
    fn Star_SRPI_SetMachineAttachBuf(c_this: *const c_void, Machine: *mut c_void, AppBuf: *mut i8);
    fn Star_SRPI_GetMachineID(c_this: *const c_void, Machine: *mut c_void) -> u32;
    fn Star_SRPI_FindMachine(c_this: *const c_void, MachineID: u32) -> *mut c_void;
    fn Star_SRPI_RegClientMachineProcess(c_this: *const c_void, CallBackPtr: *mut c_void, Object: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegClientMachineProcess(c_this: *const c_void, CallBackPtr: *mut c_void, Object: *mut c_void, Para: usize);
    fn Star_SRPI_ClientInitError(c_this: *const c_void, Machine: *mut c_void);
    fn Star_SRPI_ClientInitOk(c_this: *const c_void, Machine: *mut c_void, ReSyncFlag: VS_BOOL, TermOldScript: *const c_char, InitNewScript: *const c_char, ClientPrivateTag: u32, ClientOPPermission: u32, ClientUploadMaxSize: u32);
    fn Star_SRPI_DeleteClient(c_this: *const c_void, Machine: *mut c_void);
    fn Star_SRPI_GetClientInfo(c_this: *const c_void, Machine: *mut c_void, ClientInfo: *mut VS_CLIENTINFO);
    fn Star_SRPI_QueryFirstClientInfo(c_this: *const c_void, ClientInfo: *mut VS_CLIENTINFO) -> VS_BOOL;
    fn Star_SRPI_QueryNextClientInfo(c_this: *const c_void, ClientInfo: *mut VS_CLIENTINFO) -> VS_BOOL;
    fn Star_SRPI_GetClientNumber(c_this: *const c_void) -> i32;
    fn Star_SRPI_GetClientQos(c_this: *const c_void, Machine: *mut c_void, QosBuf: *mut VS_CLIENTQOS);
    fn Star_SRPI_SetClientQos(c_this: *const c_void, Machine: *mut c_void, QosBuf: *mut VS_CLIENTQOS);
    fn Star_SRPI_GetServiceQos(c_this: *const c_void, QosBuf: *mut VS_CLIENTQOS);
    fn Star_SRPI_DownLoad(c_this: *const c_void, AttachObject: *mut c_void, ServerPath: *const c_char, ClientPath: *const c_char, FileName: *const c_char, CallBackProc: *const c_void, Object: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UpLoad(c_this: *const c_void, AttachObject: *mut c_void, ServerPath: *const c_char, ClientPath: *const c_char, FileName: *const c_char, CallBackProc: *const c_void, Object: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_GetFileInfo(c_this: *const c_void, InfoPtr: *mut VS_UPDOWNFILEINFO);
    fn Star_SRPI_RegFileCallBack(c_this: *const c_void, CallBackProc: *const c_void, Object: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegFileCallBack(c_this: *const c_void, CallBackProc: *const c_void, Object: *mut c_void, Para: usize);
    fn Star_SRPI_GetFileStatus(c_this: *const c_void, FileName: *mut c_char, FileInfo: *mut VS_UPDOWNFILEMSG) -> i32;
    fn Star_SRPI_GetDataStatus(c_this: *const c_void, Object: *mut c_void, UniqueDataUnitID: u32, FileInfo: *mut VS_UPDOWNFILEMSG) -> i32;
    fn Star_SRPI_QueryFirstDown(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, FileInfo: *mut VS_UPDOWNFILEMSG) -> VS_BOOL;
    fn Star_SRPI_QueryNextDown(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, FileInfo: *mut VS_UPDOWNFILEMSG) -> VS_BOOL;
    fn Star_SRPI_QueryFirstUp(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, FileInfo: *mut VS_UPDOWNFILEMSG) -> VS_BOOL;
    fn Star_SRPI_QueryNextUp(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, FileInfo: *mut VS_UPDOWNFILEMSG) -> VS_BOOL;
    fn Star_SRPI_GetStaticData(c_this: *const c_void, Object: *mut c_void, UniqueDataUnitID: u32, DataVersion: *mut VS_UUID, DataSize: *mut u32, AutoDownLoad: VS_BOOL) -> *mut i8;
    fn Star_SRPI_GetStaticDataEx(c_this: *const c_void, Object: *mut c_void, UniqueDataUnitID: u32, DataVersion: *mut VS_UUID, DataSize: *mut u32, AutoDownLoad: VS_BOOL, Token: *mut c_char) -> *mut i8;
    fn Star_SRPI_CanSetStaticData(c_this: *const c_void, Object: *mut c_void, DataSize: u32) -> VS_BOOL;
    fn Star_SRPI_SetStaticData(c_this: *const c_void, Object: *mut c_void, UniqueDataUnitID: u32, DataSize: u32, DataBuf: *mut i8, RetDataVersion: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_SetStaticDataEx(c_this: *const c_void, Object: *mut c_void, UniqueDataUnitID: u32, DataSize: *mut u32, Offset: u32, FileName: *mut c_char, RetDataVersion: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_GetStaticAppCode(c_this: *const c_void, Object: *mut c_void, StaticPersistentAttributeIndex: u8) -> u32;
    fn Star_SRPI_WaitGetStaticData(c_this: *const c_void, Object: *mut c_void, UniqueDataUnitID: u32, CallBackProc: *const c_void, Para: usize, WaitFlag: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_WaitSetStaticData(c_this: *const c_void, Object: *mut c_void, UniqueDataUnitID: u32, CallBackProc: *const c_void, Para: usize, WaitFlag: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_PackStaticData(c_this: *const c_void);
    fn Star_SRPI_ForceToSaveStatic(c_this: *const c_void);
    fn Star_SRPI_ClearStatic(c_this: *const c_void, BeforeDays: u16);
    fn Star_SRPI_GetStaticVersion(c_this: *const c_void, DataSize: u32, DataBuf: *mut i8, RetDataVersion: *mut VS_UUID);
    fn Star_SRPI_GetProgramType(c_this: *const c_void) -> u16;
    fn Star_SRPI_IsDefaultServer(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_IsWindowVisible(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_HideWindow(c_this: *const c_void);
    fn Star_SRPI_ShowWindow(c_this: *const c_void);
    fn Star_SRPI_SetWindowCaption(c_this: *const c_void, Caption: *const c_char);
    fn Star_SRPI_ExitVSSystem(c_this: *const c_void, ErrorInfo: *const c_char);
    fn Star_SRPI_IsAppActive(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_SetIdleActive(c_this: *const c_void, CreateFlag: VS_BOOL);
    fn Star_SRPI_GetVersion(c_this: *const c_void, MainVersion: *mut u8, SubVersion: *mut u8, BuildVersion: *mut u16);
    fn Star_SRPI_GetVersionInfo(c_this: *const c_void, InfoBuf: *mut c_char, InfoBufSize: i32);
    fn Star_SRPI_GetWindowHandle(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetWindowSize(c_this: *const c_void, Width: *mut i32, Height: *mut i32);
    fn Star_SRPI_SetColor(c_this: *const c_void, Text: u32, Explane: u32, ObjName: u32, AttrType: u32, Number: u32, Error: u32);
    fn Star_SRPI_SetBkColor(c_this: *const c_void, BkColor: u32);
    fn Star_SRPI_ShowStatusMenu(c_this: *const c_void, MenuShowFlag: VS_BOOL, StatusShowFlag: VS_BOOL);
    fn Star_SRPI_GetClientWndHandle(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetClientWndSize(c_this: *const c_void, Width: *mut i32, Height: *mut i32);
    fn Star_SRPI_SetClientWndSize(c_this: *const c_void, Width: i32, Height: i32);
    fn Star_SRPI_SetClientWndFocus(c_this: *const c_void, hWnd: *mut c_void, NeedAction: VS_BOOL);
    fn Star_SRPI_ClearClientWnd(c_this: *const c_void);
    fn Star_SRPI_HideClientWnd(c_this: *const c_void);
    fn Star_SRPI_ShowClientWnd(c_this: *const c_void);
    fn Star_SRPI_SetClientBkColor(c_this: *const c_void, BkColor: u32);
    fn Star_SRPI_SetRunEnv_FromChildCallBack(c_this: *const c_void, Object: *mut c_void, CallBack: *const c_void, Para: usize);
    fn Star_SRPI_SetRunEnv_FromParentCallBack(c_this: *const c_void, Object: *mut c_void, CallBack: *const c_void, Para: usize);
    fn Star_SRPI_RegRunEnv_FromParentCallBack(c_this: *const c_void, Object: *mut c_void, ParentObject: *mut c_void, CallBack: *const c_void, Para: usize);
    fn Star_SRPI_UnRegRunEnv_FromParentCallBack(c_this: *const c_void, Object: *mut c_void, ParentObject: *mut c_void, CallBack: *const c_void, Para: usize);
    fn Star_SRPI_RunEnvToChild(c_this: *const c_void, Object: *mut c_void, DesObject: *mut c_void, RunEnvInfo: *mut StructOfVSRunEnv) -> VS_BOOL;
    fn Star_SRPI_RunEnvToParent(c_this: *const c_void, Object: *mut c_void, RunEnvInfo: *mut StructOfVSRunEnv) -> VS_BOOL;
    fn Star_SRPI_SetMessageHook(c_this: *const c_void, HookProc: *const c_void);
    fn Star_SRPI_GetMessageHook(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_RegLuaFunc(c_this: *const c_void, Object: *mut c_void, FuncName: *const c_char, FuncAddress: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegLuaFunc(c_this: *const c_void, Object: *mut c_void, FuncName: *const c_char, FuncAddress: *mut c_void, Para: usize);
    fn Star_SRPI_UnRegLuaFuncEx(c_this: *const c_void, Object: *mut c_void, FuncAddress: *mut c_void, Para: usize);
    fn Star_SRPI_ValidRegLuaFunc(c_this: *const c_void, Object: *mut c_void, FuncName: *const c_char, FuncAddress: *mut c_void, Para: usize);
    fn Star_SRPI_InValidRegLuaFunc(c_this: *const c_void, Object: *mut c_void, FuncName: *const c_char, FuncAddress: *mut c_void, Para: usize);
    fn Star_SRPI_RegLuaGetValueFunc(c_this: *const c_void, Object: *mut c_void, GetValueProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_RegLuaSetValueFunc(c_this: *const c_void, Object: *mut c_void, SetValueProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegLuaGetValueFunc(c_this: *const c_void, Object: *mut c_void, GetValueProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegLuaSetValueFunc(c_this: *const c_void, Object: *mut c_void, SetValueProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_ValidLuaGetValueFunc(c_this: *const c_void, Object: *mut c_void, GetValueProc: *const c_void, Para: usize);
    fn Star_SRPI_ValidLuaSetValueFunc(c_this: *const c_void, Object: *mut c_void, SetValueProc: *const c_void, Para: usize);
    fn Star_SRPI_InValidLuaGetValueFunc(c_this: *const c_void, Object: *mut c_void, GetValueProc: *const c_void, Para: usize);
    fn Star_SRPI_InValidLuaSetValueFunc(c_this: *const c_void, Object: *mut c_void, SetValueProc: *const c_void, Para: usize);
    fn Star_SRPI_GetLua(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_DoBuffer(c_this: *const c_void, ScriptInterface: *const c_char, ScriptBuf: *const i8, ScriptBufSize: i32, ModuleName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_DoFile(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LuaNewUserData(c_this: *const c_void, Size: i32);
    fn Star_SRPI_LuaSetUserDataGC(c_this: *const c_void, GCProc: *const c_void);
    fn Star_SRPI_LuaToUserData(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPI_LuaNewTable(c_this: *const c_void);
    fn Star_SRPI_LuaGetTop(c_this: *const c_void) -> i32;
    fn Star_SRPI_LuaNext(c_this: *const c_void, Index: i32) -> i32;
    fn Star_SRPI_LuaPop(c_this: *const c_void, Index: i32);
    fn Star_SRPI_LuaPushBool(c_this: *const c_void, Value: VS_BOOL);
    fn Star_SRPI_LuaPushString(c_this: *const c_void, Value: *const c_char);
    fn Star_SRPI_LuaPushNumber(c_this: *const c_void, Value: f64);
    fn Star_SRPI_LuaPushInt(c_this: *const c_void, Value: i32);
    fn Star_SRPI_LuaPushNil(c_this: *const c_void);
    fn Star_SRPI_LuaPushObject(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_LuaPushParaPkg(c_this: *const c_void, ParaPkg: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LuaPushQueryRecord(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LuaPushRect(c_this: *const c_void, rEct: *mut VS_RECT) -> VS_BOOL;
    fn Star_SRPI_LuaPushFont(c_this: *const c_void, hFont: *mut VS_FONT) -> VS_BOOL;
    fn Star_SRPI_LuaPushTime(c_this: *const c_void, hTime: *mut VS_TIME) -> VS_BOOL;
    fn Star_SRPI_LuaPushFunction(c_this: *const c_void, FunctionAddr: *mut c_void);
    fn Star_SRPI_LuaPushClosure(c_this: *const c_void, FunctionAddr: *mut c_void, n: i32);
    fn Star_SRPI_LuaUpValueIndex(c_this: *const c_void, Index: i32) -> i32;
    fn Star_SRPI_LuaPushValue(c_this: *const c_void, Index: i32);
    fn Star_SRPI_LuaInsert(c_this: *const c_void, Index: i32);
    fn Star_SRPI_LuaRemove(c_this: *const c_void, Index: i32);
    fn Star_SRPI_LuaPCall(c_this: *const c_void, nargs: i32, nresults: i32) -> VS_BOOL;
    fn Star_SRPI_LuaRCall(c_this: *const c_void, ClientID: u32, Object: *mut c_void, ScriptName: *const c_char, nArgs: i32);
    fn Star_SRPI_LuaRCallEx(c_this: *const c_void, ExcludeClientID: u32, Object: *mut c_void, ScriptName: *const c_char, nArgs: i32);
    fn Star_SRPI_LuaSRCall(c_this: *const c_void, WaitTime: u32, ClientID: u32, Object: *mut c_void, ScriptName: *const c_char, nArgs: i32, OutArgs: *mut i32) -> VS_BOOL;
    fn Star_SRPI_LuaARCall(c_this: *const c_void, WaitTime: u32, ClientID: u32, Object: *mut c_void, CallBackProc: *mut c_void, Para: usize, ScriptName: *const c_char, nArgs: i32) -> VS_BOOL;
    fn Star_SRPI_LuaRegEvent(c_this: *const c_void, SrcObject: *mut c_void, EventID: *mut VS_UUID, Object: *mut c_void, FuncAddr: *mut c_void) -> i32;
    fn Star_SRPI_LuaUnRegEvent(c_this: *const c_void, SrcObject: *mut c_void, EventID: *mut VS_UUID, Object: *mut c_void, FuncRefValue: i32);
    fn Star_SRPI_LuaProcessEvent(c_this: *const c_void, Object: *mut c_void, EventID: *mut VS_UUID, nArgs: i32, OutArgs: *mut i32) -> VS_BOOL;
    fn Star_SRPI_LuaPostProcessEvent(c_this: *const c_void, Object: *mut c_void, EventID: *mut VS_UUID, nArgs: i32, OutArgs: *mut i32) -> VS_BOOL;
    fn Star_SRPI_LuaCall(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, nArgs: i32, nOutArgs: i32) -> VS_BOOL;
    fn Star_SRPI_LuaRegHook(c_this: *const c_void, FuncAddr: *mut c_void);
    fn Star_SRPI_LuaUnRegHook(c_this: *const c_void, FuncAddr: *mut c_void);
    fn Star_SRPI_LuaType(c_this: *const c_void, Index: i32) -> i32;
    fn Star_SRPI_LuaToBool(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaToString(c_this: *const c_void, Index: i32) -> *mut c_char;
    fn Star_SRPI_LuaToNumber(c_this: *const c_void, Index: i32) -> f64;
    fn Star_SRPI_LuaToInt(c_this: *const c_void, Index: i32) -> i32;
    fn Star_SRPI_LuaToObject(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPI_LuaToParaPkg(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPI_LuaToQueryRecord(c_this: *const c_void, Index: i32) -> *mut VS_QUERYRECORD;
    fn Star_SRPI_LuaToRect(c_this: *const c_void, Index: i32, rEct: *mut VS_RECT) -> VS_BOOL;
    fn Star_SRPI_LuaToFont(c_this: *const c_void, Index: i32, hFont: *mut VS_FONT) -> VS_BOOL;
    fn Star_SRPI_LuaToTime(c_this: *const c_void, Index: i32, hTime: *mut VS_TIME) -> VS_BOOL;
    fn Star_SRPI_LuaIsBool(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsString(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsNumber(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsTable(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsNil(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsObject(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsParaPkg(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsQueryRecord(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsCFunction(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsFunction(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsFont(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsRect(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsTime(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaSetTable(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaGetTable(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaSetGlobal(c_this: *const c_void, Name: *const c_char);
    fn Star_SRPI_LuaGetGlobal(c_this: *const c_void, Name: *const c_char);
    fn Star_SRPI_LuaSetRef(c_this: *const c_void, Object: *mut c_void, Index: i32) -> i32;
    fn Star_SRPI_LuaClearRef(c_this: *const c_void, Object: *mut c_void, LuaRefValue: i32);
    fn Star_SRPI_LuaGetRef(c_this: *const c_void, Object: *mut c_void, LuaRefValue: i32);
    fn Star_SRPI_LuaInsertTable2(c_this: *const c_void, TableIndex: i32) -> i32;
    fn Star_SRPI_LuaRemoveTable2(c_this: *const c_void, TableIndex: i32, Pos: i32);
    fn Star_SRPI_LuaPushLString(c_this: *const c_void, Value: *const c_char, Len: u32);
    fn Star_SRPI_LuaObjectNewTempTable(c_this: *const c_void, Object: *mut c_void, Name: *const c_char) -> VS_BOOL;
    fn Star_SRPI_LuaObjectIsLock(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_LuaObjectLock(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_LuaObjectUnLock(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_GetValueFromLua(c_this: *const c_void, String: *const c_char) -> VS_BOOL;
    fn Star_SRPI_LuaPushBinBuf(c_this: *const c_void, BinBuf: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LuaToBinBuf(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPI_LuaIsBinBuf(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_DefLuaFunction(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_SaveToLuaFunc(c_this: *const c_void, Object: *mut c_void, LuaFileName: *const c_char, FuncName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_LuaPushSXml(c_this: *const c_void, SXml: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LuaToSXml(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPI_LuaIsSXml(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaPushFunctionPara(c_this: *const c_void, FunctionPara: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LuaToFunctionPara(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPI_LuaIsFunctionPara(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaPushCommInterface(c_this: *const c_void, CommInterface: *mut c_void, AutoRelease: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LuaToCommInterface(c_this: *const c_void, Index: i32) -> *mut c_void;
    fn Star_SRPI_LuaIsCommInterface(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaInsertTable(c_this: *const c_void, TableIndex: i32, Pos: i32);
    fn Star_SRPI_LuaRemoveTable(c_this: *const c_void, TableIndex: i32, Pos: i32);
    fn Star_SRPI_LuaObjLen(c_this: *const c_void, TableIndex: i32) -> i32;
    fn Star_SRPI_LuaGetTablei(c_this: *const c_void, TableIndex: i32, Pos: i32);
    fn Star_SRPI_LuaSetTablei(c_this: *const c_void, TableIndex: i32, Pos: i32);
    fn Star_SRPI_LuaInitObject(c_this: *const c_void, Object: *mut c_void, InitScript: *const c_char);
    fn Star_SRPI_GetAttributeLuaString(c_this: *const c_void, Object: *mut c_void) -> *mut c_char;
    fn Star_SRPI_LockGC(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_UnLockGC(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_GetObjectFromLua(c_this: *const c_void, String: *const c_char) -> *mut c_void;
    fn Star_SRPI_GCCollect(c_this: *const c_void);
    fn Star_SRPI_GetRegStr(c_this: *const c_void, SubKey: *const c_char, ValueName: *const c_char, DefaultValue: *const c_char) -> *mut c_char;
    fn Star_SRPI_GetRegInt(c_this: *const c_void, SubKey: *const c_char, ValueName: *const c_char, DefaultValue: u32) -> u32;
    fn Star_SRPI_SetupTimer(c_this: *const c_void, Ticket: i32, FunctionAddr: *const c_void, Object: *mut c_void, Para1: usize, Para2: usize, Para3: usize, Para4: usize) -> u32;
    fn Star_SRPI_KillTimer(c_this: *const c_void, TimerID: u32);
    fn Star_SRPI_GetTickCount(c_this: *const c_void) -> usize;
    fn Star_SRPI_GetMD5(c_this: *const c_void, Buf: *mut i8, BufSize: i32) -> *mut i8;
    fn Star_SRPI_MD5ToUuid(c_this: *const c_void, String: *const i8, Uuid: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_UuidToMD5(c_this: *const c_void, Uuid: *mut VS_UUID) -> *mut i8;
    fn Star_SRPI_CreateUuid(c_this: *const c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_GetSRPTempPath(c_this: *const c_void, BufSize: u32, Buf: *mut c_char);
    fn Star_SRPI_StringToUtf8(c_this: *const c_void, String: *const i8) -> *mut i8;
    fn Star_SRPI_Utf8ToString(c_this: *const c_void, String: *const i8) -> *mut i8;
    fn Star_SRPI_SetExceptHandler(c_this: *const c_void, ExceptHandlerProc: *const c_void);
    fn Star_SRPI_SRPLock(c_this: *const c_void);
    fn Star_SRPI_SRPUnLock(c_this: *const c_void);
    fn Star_SRPI_Compress(c_this: *const c_void, dest: *mut u8, destLen: *mut u32, source: *mut u8, sourceLen: u32) -> VS_BOOL;
    fn Star_SRPI_UnCompress(c_this: *const c_void, dest: *mut u8, destLen: *mut u32, source: *mut u8, sourceLen: u32) -> VS_BOOL;
    fn Star_SRPI_IsSysRootItemSync(c_this: *const c_void, SystemRootItem: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_WaitSysRootItemSync(c_this: *const c_void, SystemRootItem: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_GetAtomicService(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_CreateAtomicSysRootItem(c_this: *const c_void, SysRootItemName: *const c_char, DependSysRootItem: *const c_char, SystemRootItemID: *mut VS_UUID, SystemRootItemIDEx: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_GetAtomicSysRootItem(c_this: *const c_void, SysRootItemName: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetAtomicObject(c_this: *const c_void, UuidPtr: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_GetAtomicObjectEx(c_this: *const c_void, ParentAtomicObject: *mut c_void, ObjectName: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetAtomicClass(c_this: *const c_void, AtomicObject: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetAtomicID(c_this: *const c_void, AtomicObject: *mut c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_GetAtomicName(c_this: *const c_void, AtomicObject: *mut c_void) -> *mut c_char;
    fn Star_SRPI_AtomicToObject(c_this: *const c_void, AtomicObject: *mut c_void) -> *mut c_void;
    fn Star_SRPI_ObjectToAtomic(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_FreeAtomicObject(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_CreateAtomicObjectSimple(c_this: *const c_void, SysRootItemName: *const c_char, ObjectName: *const c_char, Attribute: *const c_char, ObjectID: *mut VS_UUID, ErrorInfo: *mut [*mut c_char;1]) -> *mut c_void;
    fn Star_SRPI_CreateAtomicObjectAttributeSimple(c_this: *const c_void, AtomicObject: *mut c_void, Attribute: *const c_char, ErrorInfo: *mut [*mut c_char;1]) -> *mut c_void;
    fn Star_SRPI_CreateAtomicStructSimple(c_this: *const c_void, StructName: *const c_char, Attribute: *const c_char, ObjectID: *mut VS_UUID, ErrorInfo: *mut [*mut c_char;1]) -> *mut c_void;
    fn Star_SRPI_CreateAtomicFunctionSimple(c_this: *const c_void, AtomicObject: *mut c_void, FunctionName: *const c_char, Attribute: *const c_char, ObjectID: *mut VS_UUID, ErrorInfo: *mut [*mut c_char;1], StdCallFlag: VS_BOOL, GlobalFunctionFlag: VS_BOOL) -> *mut c_void;
    fn Star_SRPI_SetAtomicFunction(c_this: *const c_void, AtomicFunction: *mut c_void, FuncAddress: *mut c_void);
    fn Star_SRPI_CreateAtomicMacro(c_this: *const c_void, MacroName: *const c_char, MacroType: u8) -> *mut c_void;
    fn Star_SRPI_CreateAtomicMacroItem(c_this: *const c_void, MacroObject: *mut c_void, MacroItemName: *const c_char, MacroItemValue: *const c_char) -> *mut c_void;
    fn Star_SRPI_CreateAtomicModule(c_this: *const c_void, ModuleName: *const c_char, ModuleType: u16, ModuleID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_CreateAtomicEditModule(c_this: *const c_void, ModuleName: *const c_char, ModuleID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_CreateAtomicStruct(c_this: *const c_void, StructName: *const c_char, StructCaption: *const c_char, StructID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_CreateAtomicObject(c_this: *const c_void, AtomicObject: *mut c_void, AtomicAttributeIndex: u8, AtomicClassObject: *mut c_void, ObjectName: *const c_char, ObjectID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_CreateAtomicAttachAttribute(c_this: *const c_void, AtomicObject: *mut c_void, AttributeName: *const c_char, Caption: *const c_char, Type: u8, StaticID: u32, SyncFlag: u8, CreateFlag: u8, NotifyFlag: u8, EditType: u8, EditControl: u8, EditReadOnly: u8, Default: *const c_char, Desc: *const c_char) -> *mut c_void;
    fn Star_SRPI_CreateAtomicAttribute(c_this: *const c_void, AtomicObject: *mut c_void, AttributeName: *const c_char, Caption: *const c_char, Type: u8, StaticID: u32, SyncFlag: u8, CreateFlag: u8, NotifyFlag: u8, EditType: u8, EditControl: u8, EditReadOnly: u8, Default: *const c_char, Desc: *const c_char) -> *mut c_void;
    fn Star_SRPI_CreateAtomicFuncRetAttribute(c_this: *const c_void, AtomicObject: *mut c_void, Type: u8, Desc: *const c_char) -> *mut c_void;
    fn Star_SRPI_CreateAtomicFuncParaAttribute(c_this: *const c_void, AtomicObject: *mut c_void, AttributeName: *const c_char, AttributeCaption: *const c_char, Type: u8, Desc: *const c_char) -> *mut c_void;
    fn Star_SRPI_CreateAtomicStructAttribute(c_this: *const c_void, AtomicObject: *mut c_void, AttributeName: *const c_char, Caption: *const c_char, Type: u8, Desc: *const c_char) -> *mut c_void;
    fn Star_SRPI_SetAtomicAttributeLength(c_this: *const c_void, AtomicObject: *mut c_void, Length: i32) -> VS_BOOL;
    fn Star_SRPI_SetAtomicAttributeStruct(c_this: *const c_void, AtomicObject: *mut c_void, AtomicStruct: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_SetAtomicAttributeCombobox(c_this: *const c_void, AtomicObject: *mut c_void, MacroName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_SetAtomicAttributeSyncFlag(c_this: *const c_void, AtomicObject: *mut c_void, SyncFlag: u8) -> VS_BOOL;
    fn Star_SRPI_CreateAtomicScript(c_this: *const c_void, AtomicObject: *mut c_void, ScriptName: *const c_char, ScriptID: *mut VS_UUID, Desc: *const c_char, ScriptBuf: *const i8) -> *mut c_void;
    fn Star_SRPI_CreateAtomicFunction(c_this: *const c_void, AtomicObject: *mut c_void, FunctionName: *const c_char, FunctionID: *mut VS_UUID, Desc: *const c_char, CantOvl: VS_BOOL, CallBack: VS_BOOL, StdCallFlag: VS_BOOL, GlobalFunctionFlag: VS_BOOL) -> *mut c_void;
    fn Star_SRPI_CreateAtomicLuaFunction(c_this: *const c_void, AtomicObject: *mut c_void, LuaFunctionName: *const c_char, LuaFunctionID: *mut VS_UUID, Desc: *const c_char) -> *mut c_void;
    fn Star_SRPI_CreateAtomicOvlFunction(c_this: *const c_void, AtomicObject: *mut c_void, FunctionName: *const c_char, OriginFunctionName: *const c_char, OvlFunctionID: *mut VS_UUID, Desc: *const c_char, CantOvl: VS_BOOL) -> *mut c_void;
    fn Star_SRPI_CreateAtomicFunctionEx(c_this: *const c_void, AtomicObject: *mut c_void, FunctionName: *const c_char, FunctionID: *mut VS_UUID, Desc: *const c_char, CantOvl: VS_BOOL, CallBack: VS_BOOL, Type: *const c_char, ErrorInfo: *mut [*mut c_char;1], StdCallFlag: VS_BOOL, GlobalFunctionFlag: VS_BOOL) -> *mut c_void;
    fn Star_SRPI_CreateAtomicInEvent(c_this: *const c_void, AtomicObject: *mut c_void, InEventName: *const c_char, InEventID: *mut VS_UUID, OutEventName: *const c_char) -> *mut c_void;
    fn Star_SRPI_CreateAtomicOutEvent(c_this: *const c_void, AtomicObject: *mut c_void, OutEventName: *const c_char, OutEventID: *mut VS_UUID, Desc: *const c_char, DynamicFlag: VS_BOOL) -> *mut c_void;
    fn Star_SRPI_GetAtomicFunction(c_this: *const c_void, FunctionID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_GetAtomicFunctionEx(c_this: *const c_void, AtomicObject: *mut c_void, FunctionID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_GetAtomicFunctionByName(c_this: *const c_void, AtomicObject: *mut c_void, FunctionName: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetAtomicScript(c_this: *const c_void, AtomicObject: *mut c_void, ScriptName: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetAtomicOutEvent(c_this: *const c_void, AtomicObject: *mut c_void, OutEventName: *const c_char) -> *mut c_void;
    fn Star_SRPI_GetAtomicInfo(c_this: *const c_void, Atomic: *mut c_void, AtomicType: *mut u32, Para1: *mut usize, Para2: *mut usize, Para3: *mut usize, Para4: *mut usize, Para5: *mut usize, Para6: *mut usize, Para7: *mut usize) -> VS_BOOL;
    fn Star_SRPI_GetAtomicAttributeInfo(c_this: *const c_void, AtomicObject: *mut c_void, AttributeIndexNumber: i32, AttributeIndex: *mut u8, AttributeName: *const c_char, AttributeInfo: *mut VS_ATTRIBUTEINFO) -> VS_BOOL;
    fn Star_SRPI_GetAtomicAttributeInfoEx(c_this: *const c_void, AtomicObject: *mut c_void, AttributeIndexNumber: i32, AttributeIndex: *mut u8, ThisAtomicAttributeIndex: u8, AttributeInfo: *mut VS_ATTRIBUTEINFO) -> VS_BOOL;
    fn Star_SRPI_GetAtomicAttachAttributeNumber(c_this: *const c_void, AtomicObject: *mut c_void) -> i32;
    fn Star_SRPI_GetAtomicAttachAttributeSize(c_this: *const c_void, AtomicObject: *mut c_void) -> i32;
    fn Star_SRPI_GetAtomicAttachAttributeInfoEx(c_this: *const c_void, AtomicObject: *mut c_void, AttachAttributeIndex: u8, AttributeInfo: *mut VS_ATTRIBUTEINFO) -> VS_BOOL;
    fn Star_SRPI_GetAtomicAttributeLength(c_this: *const c_void, AtomicObject: *mut c_void, Length: *mut i32) -> VS_BOOL;
    fn Star_SRPI_GetAtomicAttributeStruct(c_this: *const c_void, AtomicObject: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetAtomicAttributeCombobox(c_this: *const c_void, AtomicObject: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetAtomicAttributeSyncFlag(c_this: *const c_void, AtomicObject: *mut c_void) -> u8;
    fn Star_SRPI_ToAttributeIndex(c_this: *const c_void, AtomicObject: *mut c_void, AtomicAttributeIndex: u8) -> u8;
    fn Star_SRPI_ToAtomicAttributeIndex(c_this: *const c_void, AtomicObject: *mut c_void, AttributeIndex: u8) -> u8;
    fn Star_SRPI_GetAtomicStructAttributeNumber(c_this: *const c_void, AtomicObject: *mut c_void) -> i32;
    fn Star_SRPI_GetAtomicStructAttributeSize(c_this: *const c_void, AtomicObject: *mut c_void) -> i32;
    fn Star_SRPI_GetAtomicStructAttributeInfo(c_this: *const c_void, AtomicObject: *mut c_void, AttributeName: *const c_char, AttributeInfo: *mut VS_ATTRIBUTEINFO) -> VS_BOOL;
    fn Star_SRPI_GetAtomicStructAttributeInfoEx(c_this: *const c_void, AtomicObject: *mut c_void, ThisAtomicAttributeIndex: u8, AttributeInfo: *mut VS_ATTRIBUTEINFO) -> VS_BOOL;
    fn Star_SRPI_GetAtomicFuncRetAttributeNumber(c_this: *const c_void, AtomicObject: *mut c_void) -> i32;
    fn Star_SRPI_GetAtomicFuncParaAttributeNumber(c_this: *const c_void, AtomicObject: *mut c_void) -> i32;
    fn Star_SRPI_QueryFirstAtomicMacro(c_this: *const c_void, QueryContext: *mut usize, ServiceID: *mut VS_UUID, MacroName: *mut [*mut c_char;1], Type: *mut u8) -> *mut c_void;
    fn Star_SRPI_QueryNextAtomicMacro(c_this: *const c_void, QueryContext: *mut usize, ServiceID: *mut VS_UUID, MacroName: *mut [*mut c_char;1], Type: *mut u8) -> *mut c_void;
    fn Star_SRPI_QueryFirstAtomicInfo(c_this: *const c_void, QueryContext: *mut usize, AtomicType: u8, Para1: *mut usize, Para2: *mut usize, Para3: *mut usize, Para4: *mut usize, Para5: *mut usize) -> *mut c_void;
    fn Star_SRPI_QueryNextAtomicInfo(c_this: *const c_void, QueryContext: *mut usize, AtomicType: u8, Para1: *mut usize, Para2: *mut usize, Para3: *mut usize, Para4: *mut usize, Para5: *mut usize) -> *mut c_void;
/*  fn Star_SRPI_AtomicObjectCanOutput() -> VS_BOOL; */
/*  fn Star_SRPI_AtomicObjectAttributeCanOutput() -> VS_BOOL; */
    fn Star_SRPI_SetAtomicAttribute(c_this: *const c_void, AtomicObject: *mut c_void, AttributeIndexNumber: i32, AttributeIndex: *mut u8, ThisAtomicAttributeIndex: u8, NewValue: *const i8) -> VS_BOOL;
    fn Star_SRPI_GetAtomicAttribute(c_this: *const c_void, AtomicObject: *mut c_void, AttributeIndexNumber: i32, AttributeIndex: *mut u8, ThisAtomicAttributeIndex: u8) -> *mut c_void;
    fn Star_SRPI_GetAtomicAttributeDefault(c_this: *const c_void, AtomicObject: *mut c_void, AttributeIndexNumber: i32, AttributeIndex: *mut u8, ThisAtomicAttributeIndex: u8) -> *mut c_void;
    fn Star_SRPI_GetAtomicObjectSyncGroup(c_this: *const c_void, AtomicObject: *mut c_void) -> u32;
    fn Star_SRPI_SetAtomicObjectSyncGroup(c_this: *const c_void, AtomicObject: *mut c_void, SyncGroup: u32) -> VS_BOOL;
    fn Star_SRPI_GetAtomicObjectAttribute(c_this: *const c_void, AtomicObject: *mut c_void, SysEvent: *mut VS_BOOL, SpecialEvent: *mut u8, ActiveCmd: *mut u8, SaveFlag: *mut u8) -> VS_BOOL;
    fn Star_SRPI_SetAtomicObjectAttribute(c_this: *const c_void, AtomicObject: *mut c_void, SysEvent: VS_BOOL, SpecialEvent: u8, ActiveCmd: u8, SaveFlag: u8) -> VS_BOOL;
    fn Star_SRPI_CreateAtomicDepend(c_this: *const c_void, DependServiceName: *const c_char, DependID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_IsValid(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_ProgramRestart(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_HttpDownLoad(c_this: *const c_void, AttachObjectID: *mut VS_UUID, ServerUrl: *const c_char, ClientPath: *const c_char, FileName: *const c_char, CallBackProc: *const c_void, ObjectID: *mut VS_UUID, Para: usize, SaveFileFlag: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_HttpDownLoadAbort(c_this: *const c_void);
    fn Star_SRPI_RegWebDownFunction(c_this: *const c_void, CallBackProc: *const c_void, Para: usize);
    fn Star_SRPI_UnRegWebDownFunction(c_this: *const c_void, CallBackProc: *const c_void, Para: usize);
    fn Star_SRPI_WebDownPrint(c_this: *const c_void, uMes: u32, FileName: *const c_char, MaxLength: u64, CurLength: u64);
    fn Star_SRPI_RegDllCallBack(c_this: *const c_void, MsgCallBackProc: *const c_void, MsgCallBackPara: usize);
    fn Star_SRPI_UnRegDllCallBack(c_this: *const c_void, MsgCallBackProc: *const c_void, MsgCallBackPara: usize);
    fn Star_SRPI_AllocQueue(c_this: *const c_void, ParentObject: *mut c_void, Object: *mut c_void) -> u8;
    fn Star_SRPI_AllocQueueEx(c_this: *const c_void, ParentObject: *mut c_void, ClassID: *mut VS_UUID) -> u8;
    fn Star_SRPI_GetPeerIP(c_this: *const c_void, ClientID: u32, ClientIP: *mut VSSOCKADDR_IN) -> VS_BOOL;
    fn Star_SRPI_GetServerID(c_this: *const c_void) -> u32;
    fn Star_SRPI_RemoteSend(c_this: *const c_void, Object: *mut c_void, ClientID: u32, ParaPkg: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_GetSysDocClass(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_FirstDoc(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, DocName: *mut [*mut c_char;1]) -> *mut c_void;
    fn Star_SRPI_NextDoc(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD, DocName: *mut [*mut c_char;1]) -> *mut c_void;
    fn Star_SRPI_RegisterDoc(c_this: *const c_void, DocObject: *mut c_void, DocName: *const c_char);
    fn Star_SRPI_UnRegisterDoc(c_this: *const c_void, DocObject: *mut c_void);
    fn Star_SRPI_ProcessSysDocEvent(c_this: *const c_void, DocObjectID: *mut VS_UUID, EventID: *mut VS_UUID, RequestParam: *mut VS_EVENTPARAM_RUNPARAM) -> *mut VS_EVENTPARAM_RUNPARAM;
    fn Star_SRPI_RegDocEventFunction(c_this: *const c_void, DocObjectID: *mut VS_UUID, EventID: *mut VS_UUID, FuncAddr: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegDocEventFunction(c_this: *const c_void, DocObjectID: *mut VS_UUID, EventID: *mut VS_UUID, FuncAddr: *mut c_void, Para: usize);
    fn Star_SRPI_GetActiveServiceID(c_this: *const c_void, UuidPtr: *mut VS_UUID);
    fn Star_SRPI_RegisterAttachClass(c_this: *const c_void, OriginClass: *mut c_void, AttachClass: *mut c_void);
    fn Star_SRPI_UnRegisterAttachClass(c_this: *const c_void, OriginClass: *mut c_void, AttachClass: *mut c_void);
    fn Star_SRPI_WaitEvent(c_this: *const c_void, SrcObject: *mut c_void, EventID: *mut VS_UUID, Object: *mut c_void, FuncAddr: *mut c_void, Para: usize, AutoDelete: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_UnWaitEvent(c_this: *const c_void, SrcObject: *mut c_void, EventID: *mut VS_UUID, Object: *mut c_void, FuncAddr: *mut c_void, Para: usize);
    fn Star_SRPI_LuaPushEventPara(c_this: *const c_void, EventParam: *mut c_void) -> i32;
    fn Star_SRPI_RegChangeCallBackEx(c_this: *const c_void, Object: *mut c_void, ObjectChangeNotifyProc: *const c_void, DesObject: *mut c_void, Para: usize, ChildNotify: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_UnRegChangeCallBackEx(c_this: *const c_void, Object: *mut c_void, ObjectChangeNotifyProc: *const c_void, DesObject: *mut c_void, Para: usize);
    fn Star_SRPI_ToClipBoard(c_this: *const c_void, Info: *const c_char);
    fn Star_SRPI_FromClipBoard(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPI_IsWindowlessSite(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_RegWindowlessSiteCallBack(c_this: *const c_void, CallBackInfo: *mut VSWindowlessSiteCallBackInfo, ObjectID: *mut VS_UUID, Para: usize);
    fn Star_SRPI_UnRegWindowlessSiteCallBack(c_this: *const c_void, CallBackInfo: *mut VSWindowlessSiteCallBackInfo, ObjectID: *mut VS_UUID, Para: usize);
    fn Star_SRPI_IsWindowlessTransparent(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_Windowless_Redraw(c_this: *const c_void, fErase: VS_BOOL);
/*  fn Star_SRPI_Windowless_GetDC(); */
    fn Star_SRPI_Windowless_ReleaseDC(c_this: *const c_void, hDC: *mut c_void);
    fn Star_SRPI_KillClientWndFocus(c_this: *const c_void, hWnd: *mut c_void, NeedAction: VS_BOOL);
    fn Star_SRPI_XmlToSysRootItem(c_this: *const c_void, SXMLInterface: *mut c_void, DataPath: *const c_char, SegmentName: *const c_char, PrintProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_XmlToObject(c_this: *const c_void, SXMLInterface: *mut c_void, ParentObject: *mut c_void, AttributeIndex: u8, DataPath: *const c_char, SegmentName: *const c_char, PrintProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_ServiceToXml(c_this: *const c_void, SXMLInterface: *mut c_void, PassWord: *const c_char, DataPath: *const c_char, CFunctionFlag: VS_BOOL, OutputObjectID: VS_BOOL, PrintProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_SysRootItemToXml(c_this: *const c_void, SXMLInterface: *mut c_void, SysRootItemName: *const c_char, DataPath: *const c_char, CFunctionFlag: VS_BOOL, OutputObjectID: VS_BOOL, PrintProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_ObjectToXml(c_this: *const c_void, SXMLInterface: *mut c_void, Object: *mut c_void, DataPath: *const c_char, CFunctionFlag: VS_BOOL, OutputObjectID: VS_BOOL, PrintProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_GetVSObjectID(c_this: *const c_void, Which: i32) -> *mut VS_UUID;
    fn Star_SRPI_GetBasicInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetSXMLInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetFunctionParaInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetSRPLockInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetSRPBinBufInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetParaPkgInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetEnvMemoryFile(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetCommInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetFileDiskInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_GetSRPConfigPath(c_this: *const c_void, BufSize: u32, Buf: *mut c_char);
    fn Star_SRPI_RegTempFile(c_this: *const c_void, TempFileName: *const c_char, OriFileName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_GetRegTempFile(c_this: *const c_void, OriFileName: *const c_char, Buf: *mut c_char, BufSize: i32) -> *mut c_char;
    fn Star_SRPI_UnRegTempFile(c_this: *const c_void, TempFileName: *const c_char);
    fn Star_SRPI_QueryInterface(c_this: *const c_void, InterfaceID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_LockLuaTable(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_UnLockLuaTable(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_GetIDEx(c_this: *const c_void, Object: *mut c_void) -> *mut VS_UUID;
    fn Star_SRPI_IsRootService(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_LuaGetObjectValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char) -> VS_BOOL;
    fn Star_SRPI_LuaSetObjectValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char) -> VS_BOOL;
    fn Star_SRPI_GetSRPInterface(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetSRPInterfaceEx(c_this: *const c_void, ObjectID: *mut VS_UUID) -> *mut c_void;
    fn Star_SRPI_IsThisServiceEx(c_this: *const c_void, ObjectID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_AddRef(c_this: *const c_void);
    fn Star_SRPI_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPI_SetLog(c_this: *const c_void, Object: *mut c_void, Flag: VS_BOOL);
    fn Star_SRPI_SetLogFile(c_this: *const c_void, FileName: *const c_char);
    fn Star_SRPI_GetLogFile(c_this: *const c_void) -> *mut c_char;
    fn Star_SRPI_ClearLog(c_this: *const c_void);
    fn Star_SRPI_ApplyLog(c_this: *const c_void) -> VS_BOOL;
/*  fn Star_SRPI_Call() -> usize; */
/*  fn Star_SRPI_CallVar() -> usize; */
/*  fn Star_SRPI_FCall() -> f32; */
/*  fn Star_SRPI_FCallVar() -> f32; */
/*  fn Star_SRPI_Set() -> VS_BOOL; */
/*  fn Star_SRPI_SetEx() -> VS_BOOL; */
/*  fn Star_SRPI_SetVar() -> VS_BOOL; */
/*  fn Star_SRPI_SetVarEx() -> VS_BOOL; */
    fn Star_SRPI_Get(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> usize;
    fn Star_SRPI_GetEx(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8) -> usize;
    fn Star_SRPI_FGet(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> f32;
    fn Star_SRPI_FGetEx(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8) -> f32;
    fn Star_SRPI_LuaToLString(c_this: *const c_void, index: i32, len: *mut u32) -> *mut c_char;
    fn Star_SRPI_SetNameBoolValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: VS_BOOL, LocalChange: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_GetNameBoolValue(c_this: *const c_void, Object: *mut c_void, Name: *const c_char, Value: *mut VS_BOOL, DefaultValue: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_LuaIsInt(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_AtomicAttach(c_this: *const c_void, AtomicObject: *mut c_void, ShareLibName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_IsGlobalFunction(c_this: *const c_void, FunctionID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_IsGlobalFunctionEx(c_this: *const c_void, Object: *mut c_void, FunctionID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPI_LuaGetDefinedClass(c_this: *const c_void, Object: *mut c_void, ObjectID: *mut VS_UUID);
    fn Star_SRPI_LuaGetDefinedClassEx(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, ObjectID: *mut VS_UUID) -> VS_BOOL;
/*  fn Star_SRPI_ScriptCall() -> usize; */
/*  fn Star_SRPI_ScriptCallVar() -> usize; */
/*  fn Star_SRPI_ScriptFCall() -> f32; */
/*  fn Star_SRPI_ScriptFCallVar() -> f32; */
/*  fn Star_SRPI_ScriptRCall() -> VS_BOOL; */
/*  fn Star_SRPI_ScriptRCallVar() -> VS_BOOL; */
/*  fn Star_SRPI_ScriptRCallEx() -> VS_BOOL; */
/*  fn Star_SRPI_ScriptRCallExVar() -> VS_BOOL; */
/*  fn Star_SRPI_ScriptSRCall() -> usize; */
/*  fn Star_SRPI_ScriptSRCallVar() -> usize; */
/*  fn Star_SRPI_ScriptFSRCall() -> f32; */
/*  fn Star_SRPI_ScriptFSRCallVar() -> f32; */
    fn Star_SRPI_ScriptSetBool(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, Value: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_ScriptSetInt(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, Value: i32) -> VS_BOOL;
    fn Star_SRPI_ScriptSetNumber(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, Value: f32) -> VS_BOOL;
    fn Star_SRPI_ScriptSetStr(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, Value: *const c_char) -> VS_BOOL;
    fn Star_SRPI_ScriptSetObject(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, Type: u8, Value: usize) -> VS_BOOL;
    fn Star_SRPI_ScriptGetBool(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_ScriptGetInt(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> i32;
    fn Star_SRPI_ScriptGetNumber(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> f32;
    fn Star_SRPI_ScriptGetStr(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> *mut c_char;
    fn Star_SRPI_ScriptGetObject(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, RetType: *mut u8) -> usize;
    fn Star_SRPI_IsRegistered(c_this: *const c_void) -> VS_BOOL;
    fn Star_SRPI_SetVString(c_this: *const c_void, Buf: *mut VS_VSTRING, Str: *const c_char);
    fn Star_SRPI_ToVString(c_this: *const c_void, Str: *const c_char) -> *mut VS_VSTRING;
    fn Star_SRPI_CheckPassword(c_this: *const c_void, CheckFlag: VS_BOOL);
    fn Star_SRPI_ScriptGetStack(c_this: *const c_void) -> i32;
    fn Star_SRPI_ScriptSetStack(c_this: *const c_void, Top: i32) -> VS_BOOL;
    fn Star_SRPI_SetSourceScript(c_this: *const c_void, Object: *mut c_void, SourceScript: i32);
    fn Star_SRPI_GetSourceScript(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_FirstShareLib(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_char;
    fn Star_SRPI_NextShareLib(c_this: *const c_void, QueryRecord: *mut VS_QUERYRECORD) -> *mut c_char;
    fn Star_SRPI_GetShareLib(c_this: *const c_void, ShareLibName: *const c_char) -> *const c_void;
    fn Star_SRPI_FreeShareLib(c_this: *const c_void, ShareLibHandle: *const c_void);
    fn Star_SRPI_NewGroup(c_this: *const c_void) -> u32;
    fn Star_SRPI_FreeGroup(c_this: *const c_void, GroupID: u32);
    fn Star_SRPI_GroupAdd(c_this: *const c_void, GroupID: u32, Object: *mut c_void) -> i32;
    fn Star_SRPI_GroupGet(c_this: *const c_void, GroupID: u32, RefID: i32) -> *mut c_void;
    fn Star_SRPI_GroupRemove(c_this: *const c_void, GroupID: u32, RefID: i32);
    fn Star_SRPI_GroupRemoveEx(c_this: *const c_void, GroupID: u32, Object: *mut c_void);
    fn Star_SRPI_GroupClear(c_this: *const c_void, GroupID: u32, FreeObject: VS_BOOL);
    fn Star_SRPI_DoFileEx(c_this: *const c_void, ScriptInterface: *const c_char, FileName: *const c_char, ErrorInfo: *mut [*mut c_char;1], WorkDirectory: *const c_char, IsUTF8: VS_BOOL, ModuleName: *const c_char) -> VS_BOOL;
    fn Star_SRPI_SetCallSuper(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_SetCallBase(c_this: *const c_void, Object: *mut c_void, Class: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_PushCallBase(c_this: *const c_void, Object: *mut c_void, Class: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_PopCallBase(c_this: *const c_void, Object: *mut c_void, Class: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_RegGetObjectCallBack(c_this: *const c_void, CallBackProc: *const c_void, Para: usize);
    fn Star_SRPI_UnRegGetObjectCallBack(c_this: *const c_void, CallBackProc: *const c_void, Para: usize);
    fn Star_SRPI_RegGetObjectExCallBack(c_this: *const c_void, CallBackProc: *const c_void, Para: usize);
    fn Star_SRPI_UnRegGetObjectExCallBack(c_this: *const c_void, CallBackProc: *const c_void, Para: usize);
    fn Star_SRPI_LuaReplace(c_this: *const c_void, index: i32);
    fn Star_SRPI_LuaCheckStack(c_this: *const c_void, sz: i32);
    fn Star_SRPI_LuaIsUserData(c_this: *const c_void, index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaIsLightUserData(c_this: *const c_void, index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaRawEqual(c_this: *const c_void, index1: i32, index2: i32) -> i32;
    fn Star_SRPI_LuaCompare(c_this: *const c_void, index1: i32, index2: i32, op: i32) -> i32;
    fn Star_SRPI_LuaConcat(c_this: *const c_void, n: i32);
    fn Star_SRPI_LuaCopy(c_this: *const c_void, fromidx: i32, toidx: i32);
    fn Star_SRPI_LuaToPointer(c_this: *const c_void, index: i32) -> *mut c_void;
    fn Star_SRPI_LuaToCFunction(c_this: *const c_void, index: i32) -> *mut c_void;
    fn Star_SRPI_LuaPushLightUserData(c_this: *const c_void, p: *mut c_void);
    fn Star_SRPI_LuaRawGet(c_this: *const c_void, index: i32);
    fn Star_SRPI_LuaRawGeti(c_this: *const c_void, index: i32, n: i32);
    fn Star_SRPI_LuaCreateTable(c_this: *const c_void, narr: i32, nrec: i32);
    fn Star_SRPI_LuaGetMetatable(c_this: *const c_void, index: i32) -> i32;
    fn Star_SRPI_LuaSetField(c_this: *const c_void, index: i32, k: *const c_char);
    fn Star_SRPI_LuaRawSet(c_this: *const c_void, index: i32);
    fn Star_SRPI_LuaRawSeti(c_this: *const c_void, index: i32, n: i32);
    fn Star_SRPI_LuaSetMetatable(c_this: *const c_void, index: i32);
    fn Star_SRPI_Lua_PCall(c_this: *const c_void, nargs: i32, nresults: i32, msgh: i32) -> i32;
    fn Star_SRPI_Lua_Call(c_this: *const c_void, nargs: i32, nresults: i32);
    fn Star_SRPI_LuaAtPanic(c_this: *const c_void, panicf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_LuaGC(c_this: *const c_void, what: i32, data: i32) -> i32;
    fn Star_SRPI_LuaError(c_this: *const c_void) -> i32;
    fn Star_SRPI_LuaIsNoneOrNil(c_this: *const c_void, index: i32) -> i32;
    fn Star_SRPI_LuaTypeName(c_this: *const c_void, tp: i32) -> *mut c_char;
    fn Star_SRPI_LuaL_CheckAny(c_this: *const c_void, narg: i32);
    fn Star_SRPI_LuaL_CheckInt(c_this: *const c_void, narg: i32) -> i32;
    fn Star_SRPI_LuaL_CheckLong(c_this: *const c_void, narg: i32) -> i32;
    fn Star_SRPI_LuaL_CheckLString(c_this: *const c_void, narg: i32, l: *mut i32) -> *const c_char;
    fn Star_SRPI_LuaL_CheckNumber(c_this: *const c_void, narg: i32) -> f64;
    fn Star_SRPI_LuaL_CheckOption(c_this: *const c_void, narg: i32, def: *const c_char, lst: *const c_char) -> i32;
    fn Star_SRPI_LuaL_CheckStack(c_this: *const c_void, sz: i32, msg: *const c_char);
    fn Star_SRPI_LuaL_CheckString(c_this: *const c_void, narg: i32) -> *const c_char;
    fn Star_SRPI_LuaL_CheckType(c_this: *const c_void, narg: i32, t: i32);
    fn Star_SRPI_LuaL_CheckUData(c_this: *const c_void, narg: i32, tname: *const c_char) -> *mut c_void;
    fn Star_SRPI_LuaL_CheckVersion(c_this: *const c_void);
    fn Star_SRPI_LuaL_DoFile(c_this: *const c_void, filename: *const c_char) -> i32;
    fn Star_SRPI_LuaL_DoString(c_this: *const c_void, str: *const c_char) -> i32;
    fn Star_SRPI_LuaL_Error(c_this: *const c_void, info: *const c_char) -> i32;
    fn Star_SRPI_LuaL_GetMetatable(c_this: *const c_void, tname: *const c_char);
    fn Star_SRPI_LuaL_GetSubTable(c_this: *const c_void, idx: i32, fname: *const c_char) -> i32;
    fn Star_SRPI_LuaL_Len(c_this: *const c_void, index: i32) -> i32;
    fn Star_SRPI_LuaL_LoadBuffer(c_this: *const c_void, buff: *const c_char, sz: i32, name: *const c_char) -> i32;
    fn Star_SRPI_LuaL_LoadBufferx(c_this: *const c_void, buff: *const c_char, sz: i32, name: *const c_char, mode: *const c_char) -> i32;
    fn Star_SRPI_LuaL_LoadFile(c_this: *const c_void, filename: *const c_char) -> i32;
    fn Star_SRPI_LuaL_LoadFilex(c_this: *const c_void, filename: *const c_char, mode: *const c_char) -> i32;
    fn Star_SRPI_LuaL_LoadString(c_this: *const c_void, s: *const c_char) -> i32;
    fn Star_SRPI_LuaL_NewLib(c_this: *const c_void, l: *mut VSLuaL_Reg);
    fn Star_SRPI_LuaL_NewMetatable(c_this: *const c_void, tname: *const c_char) -> i32;
    fn Star_SRPI_LuaL_Ref(c_this: *const c_void, t: i32) -> i32;
    fn Star_SRPI_LuaL_Requiref(c_this: *const c_void, modname: *const c_char, openf: *mut c_void, glb: i32);
    fn Star_SRPI_LuaL_SetFuncs(c_this: *const c_void, l: *mut VSLuaL_Reg, nup: i32);
    fn Star_SRPI_LuaL_SetMetatable(c_this: *const c_void, tname: *const c_char);
    fn Star_SRPI_LuaL_TestUData(c_this: *const c_void, narg: i32, tname: *const c_char) -> *mut c_void;
    fn Star_SRPI_LuaL_ToLString(c_this: *const c_void, idx: i32, len: *mut i32) -> *const c_char;
    fn Star_SRPI_LuaL_TypeName(c_this: *const c_void, index: i32) -> *const c_char;
    fn Star_SRPI_LuaL_UnRef(c_this: *const c_void, t: i32, refval: i32);
    fn Star_SRPI_LuaL_Where(c_this: *const c_void, lvl: i32);
    fn Star_SRPI_GetControlService(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPI_RegLuaFuncFilter(c_this: *const c_void, Object: *mut c_void, Filter: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegLuaFuncFilter(c_this: *const c_void, Object: *mut c_void, Filter: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_RegNewFunctionCallBack(c_this: *const c_void, Object: *mut c_void, callback: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_UnRegNewFunctionCallBack(c_this: *const c_void, Object: *mut c_void, callback: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_IsRegLuaFunc(c_this: *const c_void, Object: *mut c_void, FuncName: *const c_char, FuncAddress: *mut c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_IMallocStaticObject(c_this: *const c_void, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_IMallocStaticObjectEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_IMallocGlobalObject(c_this: *const c_void, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void, ClientID: u32) -> *mut c_void;
    fn Star_SRPI_IMallocGlobalObjectEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void, ClientID: u32) -> *mut c_void;
    fn Star_SRPI_IMallocObject(c_this: *const c_void, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_IMallocObjectEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_IMallocObjectL(c_this: *const c_void, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_IMallocObjectLEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void) -> *mut c_void;
    fn Star_SRPI_IMallocClientObject(c_this: *const c_void, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void, ClientID: u32) -> *mut c_void;
    fn Star_SRPI_IMallocClientObjectEx(c_this: *const c_void, ObjectID: *mut VS_UUID, ParentObject: *mut c_void, AttributeIndex: u8, ObjectClassID: *mut VS_UUID, InitBuf: *mut c_void, ClientID: u32) -> *mut c_void;
    fn Star_SRPI_LoadRawModule(c_this: *const c_void, ScriptInterface: *const c_char, ModuleName: *const c_char, FileOrString: *const c_char, IsString: VS_BOOL, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPI_LoadRawModuleEx(c_this: *const c_void, ScriptInterface: *const c_char, ModuleName: *const c_char, Object: *mut c_void, ErrorInfo: *mut [*mut c_char;1]) -> VS_BOOL;
    fn Star_SRPI_AttachRawContext(c_this: *const c_void, Object: *mut c_void, ScriptInterface: *const c_char, ContextName: *const c_char, IsClass: VS_BOOL, ContextInfo: *const c_char) -> VS_BOOL;
    fn Star_SRPI_DetachRawContext(c_this: *const c_void, Object: *mut c_void, CallUnLockGC: VS_BOOL);
    fn Star_SRPI_GetRawContextType(c_this: *const c_void, Object: *mut c_void, ScriptInterface: *mut [*mut c_char;1]) -> *mut c_char;
    fn Star_SRPI_CreateRawContextBuf(c_this: *const c_void, Object: *mut c_void, ScriptInterface: *const c_char, ContextBuf: *mut i8, ContextBufSize: i32) -> VS_BOOL;
    fn Star_SRPI_GetRawContextBuf(c_this: *const c_void, Object: *mut c_void, ScriptInterface: *const c_char) -> *mut c_void;
    fn Star_SRPI_HasRawContext(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_RawContextEquals(c_this: *const c_void, Object1: *mut c_void, Object2: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_NewRawProxy(c_this: *const c_void, ScriptInterface: *const c_char, AttachObject: *mut c_void, AttachFunction: *const c_char, ProyInfo: *const c_char, ProxyType: i32) -> *mut c_void;
    fn Star_SRPI_NewRawProxyEx(c_this: *const c_void, Object: *mut c_void, ScriptInterface: *const c_char, AttachFunction: *const c_char, ProyInfo: *const c_char) -> *mut c_void;
    fn Star_SRPI_CreateRawProxyCode(c_this: *const c_void, ScriptInterface: *const c_char, NewPackageName: *const c_char, Imports: *const c_char, NewClassName: *const c_char, BaseClass: *const c_char, Methods: *const c_char, Interface: *const c_char) -> *mut c_char;
    fn Star_SRPI_ImportRawContext(c_this: *const c_void, ScriptInterface: *const c_char, ContextName: *const c_char, IsClass: VS_BOOL, ContextInfo: *const c_char) -> *mut c_void;
    fn Star_SRPI_AssignRawObject(c_this: *const c_void, Object: *mut c_void, RawObject: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetInitPara(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_NewScriptRawType(c_this: *const c_void, RawType: i32, IsParaPkg: *mut VS_BOOL) -> *mut c_void;
    fn Star_SRPI_SetScriptRawType(c_this: *const c_void, Object: *mut c_void, RawType: i32) -> VS_BOOL;
    fn Star_SRPI_GetScriptRawType(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_RegRawLuaSetValueFunc(c_this: *const c_void, Object: *mut c_void, SetValueProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_GetRawLuaSetValueFunc(c_this: *const c_void, Object: *mut c_void, ValueName: *const c_char, Para: *mut usize) -> *mut c_void;
    fn Star_SRPI_UnRegRawLuaSetValueFunc(c_this: *const c_void, Object: *mut c_void, SetValueProc: *const c_void, Para: usize) -> VS_BOOL;
    fn Star_SRPI_AddRefEx(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_DelRefEx(c_this: *const c_void, Object: *mut c_void);
    fn Star_SRPI_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPI_ReleaseOwnerEx(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_ReleaseOwnerExForScript(c_this: *const c_void, ScriptInterface: *const c_char, Object: *mut c_void) -> VS_BOOL;
    fn Star_SRPI_CaptureOwnerExForScript(c_this: *const c_void, ScriptInterface: *const c_char, Object: *mut c_void);
    fn Star_SRPI_GetRefEx(c_this: *const c_void, Object: *mut c_void) -> i32;
    fn Star_SRPI_GetRefInfo(c_this: *const c_void, Object: *mut c_void) -> *mut c_char;
    fn Star_SRPI_GetLastError(c_this: *const c_void) -> i32;
    fn Star_SRPI_GetLastErrorInfo(c_this: *const c_void, LineIndex: *mut u32, SourceName: *mut [*mut c_char;1]) -> *mut c_char;
    fn Star_SRPI_CreateAtomicFunctionSimpleEx(c_this: *const c_void, AtomicObject: *mut c_void, FunctionName: *const c_char, Attribute: *const c_char, FuncAddress: *mut c_void, ErrorInfo: *mut [*mut c_char;1]) -> *mut c_void;
    fn Star_SRPI_LuaIsFunctionDefined(c_this: *const c_void, Object: *mut c_void, FuncName: *const c_char, IncludeRawOrDefaultFunction: VS_BOOL) -> *mut c_void;
    fn Star_SRPI_LuaToRaw(c_this: *const c_void, Index: i32, IsClass: VS_BOOL) -> *mut c_void;
    fn Star_SRPI_LuaTableToParaPkg(c_this: *const c_void, Index: i32, ParaPkg: *mut c_void, TableCanBeWrap: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_ScriptSetBoolIndex(c_this: *const c_void, Object: *mut c_void, Index: i32, Value: VS_BOOL) -> VS_BOOL;
    fn Star_SRPI_ScriptSetIntIndex(c_this: *const c_void, Object: *mut c_void, Index: i32, Value: i32) -> VS_BOOL;
    fn Star_SRPI_ScriptSetNumberIndex(c_this: *const c_void, Object: *mut c_void, Index: i32, Value: f32) -> VS_BOOL;
    fn Star_SRPI_ScriptSetStrIndex(c_this: *const c_void, Object: *mut c_void, Index: i32, Value: *const c_char) -> VS_BOOL;
    fn Star_SRPI_ScriptSetObjectIndex(c_this: *const c_void, Object: *mut c_void, Index: i32, Type: u8, Value: usize) -> VS_BOOL;
    fn Star_SRPI_ScriptGetBoolIndex(c_this: *const c_void, Object: *mut c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_ScriptGetIntIndex(c_this: *const c_void, Object: *mut c_void, Index: i32) -> i32;
    fn Star_SRPI_ScriptGetNumberIndex(c_this: *const c_void, Object: *mut c_void, Index: i32) -> f32;
    fn Star_SRPI_ScriptGetStrIndex(c_this: *const c_void, Object: *mut c_void, Index: i32) -> *mut c_char;
    fn Star_SRPI_ScriptGetObjectIndex(c_this: *const c_void, Object: *mut c_void, Index: i32, RetType: *mut u8) -> usize;
    fn Star_SRPI_ScriptGetRawObject(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, RetType: *mut u8) -> usize;
    fn Star_SRPI_ScriptGetRawObjectIndex(c_this: *const c_void, Object: *mut c_void, Index: i32, RetType: *mut u8) -> usize;
    fn Star_SRPI_SetReturnRawFlag(c_this: *const c_void, Object: *mut c_void, ReturnRawFlag: VS_BOOL);
    fn Star_SRPI_GetReturnRawFlag(c_this: *const c_void, Object: *mut c_void) -> VS_BOOL;
/*  fn Star_SRPI_ScriptCall2() -> usize; */
/*  fn Star_SRPI_ScriptCallVar2() -> usize; */
/*  fn Star_SRPI_ScriptSRCall2() -> usize; */
/*  fn Star_SRPI_ScriptSRCallVar2() -> usize; */
/*  fn Star_SRPI_IMallocStaticObject2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocStaticObjectEx2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocGlobalObject2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocGlobalObjectEx2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocObject2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocObjectEx2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocObjectL2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocObjectLEx2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocClientObject2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocClientObjectEx2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocStaticObjectVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocStaticObjectExVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocGlobalObjectVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocGlobalObjectExVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocObjectVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocObjectExVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocObjectLVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocObjectLExVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocClientObjectVar2() -> *mut c_void; */
/*  fn Star_SRPI_IMallocClientObjectExVar2() -> *mut c_void; */
    fn Star_SRPI_LuaSyncCall(c_this: *const c_void, Object: *mut c_void, ScriptName: *const c_char, nArgs: i32, nOutArgs: i32) -> VS_BOOL;
/*  fn Star_SRPI_ScriptSyncCall() -> usize; */
/*  fn Star_SRPI_ScriptSyncCallVar() -> usize; */
/*  fn Star_SRPI_ScriptSyncFCall() -> f32; */
/*  fn Star_SRPI_ScriptSyncFCallVar() -> f32; */
/*  fn Star_SRPI_ScriptSyncCall2() -> usize; */
/*  fn Star_SRPI_ScriptSyncCallVar2() -> usize; */
    fn Star_SRPI_LuaPushInt64(c_this: *const c_void, Value: i64);
    fn Star_SRPI_LuaToInt64(c_this: *const c_void, Index: i32) -> i64;
    fn Star_SRPI_LuaIsInt64(c_this: *const c_void, Index: i32) -> VS_BOOL;
    fn Star_SRPI_LuaPushUWord(c_this: *const c_void, Value: usize);
    fn Star_SRPI_LuaToUWord(c_this: *const c_void, Index: i32) -> usize;
    fn Star_SRPI_LuaIsUWord(c_this: *const c_void, Index: i32) -> VS_BOOL;
/*  fn Star_SRPI_SRemoteCallInt64() -> i64; */
/*  fn Star_SRPI_SRemoteCallInt64Var() -> i64; */
/*  fn Star_SRPI_SRemoteCallDouble() -> f64; */
/*  fn Star_SRPI_SRemoteCallDoubleVar() -> f64; */
/*  fn Star_SRPI_CallDouble() -> f64; */
/*  fn Star_SRPI_CallDoubleVar() -> f64; */
/*  fn Star_SRPI_CallInt64() -> i64; */
/*  fn Star_SRPI_CallInt64Var() -> i64; */
    fn Star_SRPI_GetInt64(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> i64;
    fn Star_SRPI_GetInt64Ex(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8) -> i64;
    fn Star_SRPI_GetDouble(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> f64;
    fn Star_SRPI_GetDoubleEx(c_this: *const c_void, Object: *mut c_void, AttributeIndex: u8) -> f64;
/*  fn Star_SRPI_ScriptCallInt64() -> i64; */
/*  fn Star_SRPI_ScriptCallInt64Var() -> i64; */
/*  fn Star_SRPI_ScriptCallDouble() -> f64; */
/*  fn Star_SRPI_ScriptCallDoubleVar() -> f64; */
/*  fn Star_SRPI_ScriptSRCallInt64() -> i64; */
/*  fn Star_SRPI_ScriptSRCallInt64Var() -> i64; */
/*  fn Star_SRPI_ScriptSRCallDouble() -> f64; */
/*  fn Star_SRPI_ScriptSRCallDoubleVar() -> f64; */
    fn Star_SRPI_ScriptSetInt64(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, Value: i64) -> VS_BOOL;
    fn Star_SRPI_ScriptSetDouble(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, Value: f64) -> VS_BOOL;
    fn Star_SRPI_ScriptGetInt64(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> i64;
    fn Star_SRPI_ScriptGetDouble(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> f64;
    fn Star_SRPI_ScriptSetInt64Index(c_this: *const c_void, Object: *mut c_void, Index: i32, Value: i64) -> VS_BOOL;
    fn Star_SRPI_ScriptSetDoubleIndex(c_this: *const c_void, Object: *mut c_void, Index: i32, Value: f64) -> VS_BOOL;
    fn Star_SRPI_ScriptGetInt64Index(c_this: *const c_void, Object: *mut c_void, Index: i32) -> i64;
    fn Star_SRPI_ScriptGetDoubleIndex(c_this: *const c_void, Object: *mut c_void, Index: i32) -> f64;
    fn Star_SRPI_ScriptGetRawObjectInt64(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> i64;
    fn Star_SRPI_ScriptGetRawObjectDouble(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char) -> f64;
    fn Star_SRPI_ScriptGetRawObjectIndexInt64(c_this: *const c_void, Object: *mut c_void, Index: i32) -> i64;
    fn Star_SRPI_ScriptGetRawObjectIndexDouble(c_this: *const c_void, Object: *mut c_void, Index: i32) -> f64;
/*  fn Star_SRPI_ScriptFCall2() -> f32; */
/*  fn Star_SRPI_ScriptFCallVar2() -> f32; */
/*  fn Star_SRPI_ScriptCallInt642() -> i64; */
/*  fn Star_SRPI_ScriptCallInt64Var2() -> i64; */
/*  fn Star_SRPI_ScriptCallDouble2() -> f64; */
/*  fn Star_SRPI_ScriptCallDoubleVar2() -> f64; */
/*  fn Star_SRPI_ScriptFSRCall2() -> f32; */
/*  fn Star_SRPI_ScriptFSRCallVar2() -> f32; */
/*  fn Star_SRPI_ScriptSRCallInt642() -> i64; */
/*  fn Star_SRPI_ScriptSRCallInt64Var2() -> i64; */
/*  fn Star_SRPI_ScriptSRCallDouble2() -> f64; */
/*  fn Star_SRPI_ScriptSRCallDoubleVar2() -> f64; */
/*  fn Star_SRPI_ScriptSyncCallInt64() -> i64; */
/*  fn Star_SRPI_ScriptSyncCallInt64Var() -> i64; */
/*  fn Star_SRPI_ScriptSyncCallDouble() -> f64; */
/*  fn Star_SRPI_ScriptSyncCallDoubleVar() -> f64; */
/*  fn Star_SRPI_ScriptSyncFCall2() -> f32; */
/*  fn Star_SRPI_ScriptSyncFCallVar2() -> f32; */
/*  fn Star_SRPI_ScriptSyncCallInt642() -> i64; */
/*  fn Star_SRPI_ScriptSyncCallInt64Var2() -> i64; */
/*  fn Star_SRPI_ScriptSyncCallDouble2() -> f64; */
/*  fn Star_SRPI_ScriptSyncCallDoubleVar2() -> f64; */
    fn Star_SRPI_ScriptGetObjectEx(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, RetType: *mut u8, RetDouble: *mut f64, RetInt64: *mut i64) -> usize;
    fn Star_SRPI_ScriptGetObjectIndexEx(c_this: *const c_void, Object: *mut c_void, Index: i32, RetType: *mut u8, RetDouble: *mut f64, RetInt64: *mut i64) -> usize;
    fn Star_SRPI_ScriptGetRawObjectEx(c_this: *const c_void, Object: *mut c_void, AttributeName: *const c_char, RetType: *mut u8, RetDouble: *mut f64, RetInt64: *mut i64) -> usize;
    fn Star_SRPI_ScriptGetRawObjectIndexEx(c_this: *const c_void, Object: *mut c_void, Index: i32, RetType: *mut u8, RetDouble: *mut f64, RetInt64: *mut i64) -> usize;
    fn Star_SRPI_RemoteCallRspInt64(c_this: *const c_void, Object: *mut c_void, ClientID: u32, RemoteCallID: u32, RemoteCallName: *const c_char, RemoteSourceTag: u16, RetCode: u32, RetValue: i64, RemoteAttach: *mut c_void);
    fn Star_SRPI_RemoteCallRspDouble(c_this: *const c_void, Object: *mut c_void, ClientID: u32, RemoteCallID: u32, RemoteCallName: *const c_char, RemoteSourceTag: u16, RetCode: u32, RetValue: f64, RemoteAttach: *mut c_void);
    fn Star_SRPI_RawToParaPkg(c_this: *const c_void, Object: *mut c_void) -> *mut c_void;
    fn Star_SRPI_GetNameEx(c_this: *const c_void, Object: *mut c_void) -> *mut c_char;
    fn Star_SRPMF_Release(c_this: *const c_void);
    fn Star_SRPMF_GetNumber(c_this: *const c_void) -> i32;
    fn Star_SRPMF_InsertFile(c_this: *const c_void, FileName: *const c_char, FileBuf: *mut u8, FileBufSize: u32, FileID: *mut VS_UUID) -> VS_BOOL;
    fn Star_SRPMF_SetFromDisk(c_this: *const c_void, FileListInfo: *mut c_char, DiskFileName: *const c_char, FileStartOffset: u32) -> VS_BOOL;
    fn Star_SRPMF_SetFromMemory(c_this: *const c_void, FileListInfo: *mut c_char, FileMemory: *mut u8, FileStartOffset: u32) -> VS_BOOL;
    fn Star_SRPMF_IsExist(c_this: *const c_void, FileName: *const c_char) -> VS_BOOL;
    fn Star_SRPMF_GetSize(c_this: *const c_void, FileName: *const c_char) -> u32;
    fn Star_SRPMF_Read(c_this: *const c_void, FileName: *const c_char, ReadBuf: *mut u8) -> u32;
    fn Star_SRPMF_AddRef(c_this: *const c_void);
    fn Star_SRPMF_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPMF_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPFD_Release(c_this: *const c_void);
    fn Star_SRPFD_Open(c_this: *const c_void, SectorNumberPerCluster: u32, FileName: *const c_char, CreateFlag: VS_BOOL) -> VS_BOOL;
    fn Star_SRPFD_GetEmptySector(c_this: *const c_void) -> u32;
    fn Star_SRPFD_ClearSectorList(c_this: *const c_void, SectorNumber: i32, SectorIndex: *mut u32);
    fn Star_SRPFD_FlushDirtySector(c_this: *const c_void);
    fn Star_SRPFD_IsSectorEmpty(c_this: *const c_void, SectorIndex: u32) -> VS_BOOL;
    fn Star_SRPFD_SetSectorInUse(c_this: *const c_void, SectorIndex: u32) -> VS_BOOL;
    fn Star_SRPFD_WriteSector(c_this: *const c_void, SectorIndex: u32, DataHeader: *mut u8, NextSector: u32, DataOffset: u32, DataSize: u32, DataBuf: *mut i8) -> u32;
    fn Star_SRPFD_ReadSector(c_this: *const c_void, SectorIndex: u32, DataHeader: *mut u8, NextSector: *mut u32, DataOffset: u32, DataSize: u32, DataBuf: *mut i8) -> u32;
    fn Star_SRPFD_ReadFileHeader(c_this: *const c_void, Buf: *mut u8) -> VS_BOOL;
    fn Star_SRPFD_SaveFileHeader(c_this: *const c_void, Buf: *mut u8) -> VS_BOOL;
    fn Star_SRPFD_GetNextSectorIndex(c_this: *const c_void, SectorIndex: u32, DataHeader: *mut u8, NextSector: *mut u32) -> VS_BOOL;
    fn Star_SRPFD_GetSize(c_this: *const c_void, DiskSize: *mut u32) -> u32;
    fn Star_SRPFD_Clear(c_this: *const c_void);
    fn Star_SRPFD_Close(c_this: *const c_void);
    fn Star_SRPFD_AddRef(c_this: *const c_void);
    fn Star_SRPFD_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPFD_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPCS_Release(c_this: *const c_void);
    fn Star_SRPCS_AnsiToUnicode(c_this: *const c_void, CodePage: u32, ToCode: *const c_char, AnsiStr: *const i8, StrLength: i32) -> *mut i8;
    fn Star_SRPCS_UnicodeToAnsi(c_this: *const c_void, FromCode: *const c_char, CodePage: u32, WideStr: *const i8, StrLength: i32, BytesPerChar: i32) -> *mut i8;
    fn Star_SRPCS_AnsiToUTF8(c_this: *const c_void, AnsiStr: *const i8, StrLength: i32) -> *mut i8;
    fn Star_SRPCS_UTF8ToAnsi(c_this: *const c_void, UTFStr: *const i8, StrLength: i32) -> *mut i8;
    fn Star_SRPCS_ToOSPlatString(c_this: *const c_void, Str: *const i8, StrLength: i32) -> *mut i8;
    fn Star_SRPCS_ToOSPlatStringEx(c_this: *const c_void, OSType: u32, Str: *const i8, StrLength: i32) -> *mut i8;
    fn Star_SRPCS_FreeBuf(c_this: *const c_void, Buf: *mut c_void);
    fn Star_SRPCS_vs_reg_beginlock(c_this: *const c_void);
    fn Star_SRPCS_vs_reg_endlock(c_this: *const c_void);
/*  fn Star_SRPCS_vs_reg_createkeyex() -> u32; */
/*  fn Star_SRPCS_vs_reg_closekey() -> u32; */
/*  fn Star_SRPCS_vs_reg_setvalue() -> u32; */
/*  fn Star_SRPCS_vs_reg_openkeyex() -> u32; */
/*  fn Star_SRPCS_vs_reg_queryvalueex() -> u32; */
/*  fn Star_SRPCS_vs_reg_deletekeyex() -> u32; */
/*  fn Star_SRPCS_vs_reg_enumkey() -> u32; */
    fn Star_SRPCS_GetCharBytes(c_this: *const c_void, Buf: *const i8) -> i32;
    fn Star_SRPCS_ToAnsi(c_this: *const c_void, Locale: *const c_char, Str: *const i8) -> *mut i8;
    fn Star_SRPCS_FromAnsi(c_this: *const c_void, Locale: *const c_char, Str: *const i8) -> *mut i8;
    fn Star_SRPCS_vs_iconv(c_this: *const c_void, FromLocale: *const c_char, ToLocale: *const c_char, InBuf: *const i8, InBufSize: i32, RetBufSize: *mut i32) -> *mut i8;
    fn Star_SRPCS_AnsiToUnicodeEx(c_this: *const c_void, CodePage: u32, ToCode: *const c_char, AnsiStr: *const i8, StrLength: i32, RetCharLength: *mut i32) -> *mut i8;
    fn Star_SRPCS_UnicodeToAnsiEx(c_this: *const c_void, FromCode: *const c_char, CodePage: u32, WideStr: *const i8, StrLength: i32, BytesPerChar: i32, RetCharLength: *mut i32) -> *mut i8;
    fn Star_SRPCS_AnsiToUTF8Ex(c_this: *const c_void, AnsiStr: *const i8, StrLength: i32, RetCharLength: *mut i32) -> *mut i8;
    fn Star_SRPCS_UTF8ToAnsiEx(c_this: *const c_void, UTFStr: *const i8, StrLength: i32, RetCharLength: *mut i32) -> *mut i8;
    fn Star_SRPFP_Release(c_this: *const c_void);
    fn Star_SRPFP_Clear(c_this: *const c_void);
    fn Star_SRPFP_GetNumber(c_this: *const c_void) -> i32;
    fn Star_SRPFP_GetType(c_this: *const c_void, Index: i32) -> u8;
    fn Star_SRPFP_GetValue(c_this: *const c_void, Index: i32) -> usize;
    fn Star_SRPFP_SetValue(c_this: *const c_void, Index: i32, In_Type: u8, In_Para: usize) -> VS_BOOL;
    fn Star_SRPFP_Call(c_this: *const c_void, Object: *mut c_void, FunctionID: *mut VS_UUID, RetValue: *mut usize, RetType: *mut u8) -> VS_BOOL;
    fn Star_SRPFP_Dup(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPFP_AddRef(c_this: *const c_void);
    fn Star_SRPFP_GetRef(c_this: *const c_void) -> i32;
    fn Star_SRPFP_ReleaseOwner(c_this: *const c_void);
    fn Star_SRPFP_GetFloatValue(c_this: *const c_void, Index: i32) -> f32;
    fn Star_SRPFP_GetDoubleValue(c_this: *const c_void, Index: i32) -> f64;
    fn Star_SRPFP_GetInt64Value(c_this: *const c_void, Index: i32) -> i64;
    fn Star_SRPFP_SetFloatValue(c_this: *const c_void, Index: i32, In_Para: f32) -> VS_BOOL;
    fn Star_SRPFP_SetDoubleValue(c_this: *const c_void, Index: i32, In_Para: f64) -> VS_BOOL;
    fn Star_SRPFP_SetInt64Value(c_this: *const c_void, Index: i32, In_Para: i64) -> VS_BOOL;
    fn Star_SRPFP_CallEx(c_this: *const c_void, Object: *mut c_void, FunctionID: *mut VS_UUID, RetValue: *mut usize, RetDouble: *mut f64, RetInt64: *mut i64, RetType: *mut u8) -> VS_BOOL;
    fn Star_SRPFP_SetValueFromLua(c_this: *const c_void, Index: i32, LuaIndex: i32) -> VS_BOOL;
    fn Star_SRPLock_Release(c_this: *const c_void);
    fn Star_SRPLock_Lock(c_this: *const c_void);
    fn Star_SRPLock_UnLock(c_this: *const c_void);
    fn Star_StarCore_Release(c_this: *const c_void);
    fn Star_StarCore_GetInitResult(c_this: *const c_void) -> i32;
    fn Star_StarCore_SetWnd(c_this: *const c_void, In_hWnd: *mut c_void);
    fn Star_StarCore_SetWndActive(c_this: *const c_void, In_ActiveFlag: VS_BOOL);
    fn Star_StarCore_CreateService(c_this: *const c_void, ServiceName: *const c_char, ImportService: *mut VSImportServiceDef) -> *mut c_void;
    fn Star_StarCore_CreateService1(c_this: *const c_void, ServicePath: *const c_char, ServiceName: *const c_char, ServiceID: *mut VS_UUID, RootPass: *const c_char, ImportService: *mut VSImportServiceDef) -> *mut c_void;
    fn Star_StarCore_CreateService2(c_this: *const c_void, ServicePath: *const c_char, ServiceName: *const c_char, ServiceID: *mut VS_UUID, RootPass: *const c_char, FrameInterval: i32, NetPkgSize: i32, UploadPkgSize: i32, DownloadPkgSize: i32, DataUpPkgSize: i32, DataDownPkgSize: i32, ImportService: *mut VSImportServiceDef) -> *mut c_void;
    fn Star_StarCore_Flush(c_this: *const c_void, WaitFlag: VS_BOOL);
    fn Star_StarCore_MsgLoop(c_this: *const c_void, SRPMsgLoopExitProc: *const c_void);
    fn Star_StarCore_GetControlInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_StarCore_GetBasicInterface(c_this: *const c_void) -> *mut c_void;
    fn Star_SRPMM_memset(Buf: *mut c_void, c: i8, Len: i32);
    fn Star_SRPMM_memcpy(DesBuf: *mut c_void, SrcBuf: *const c_void, Len: i32);
    fn Star_SRPMM_strlen(Buf: *mut i8) -> i32;
}

