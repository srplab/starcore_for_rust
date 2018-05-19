#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(unused_assignments)]
#![allow(improper_ctypes)]

/*
#![feature(libc)]
extern crate libc;
use libc::{c_int, c_char, size_t,c_void};
*/

#[cfg(target_os="android")]
use std::os::raw::{c_int, c_void};

#[cfg(target_os="android")]
type c_char = u8;

#[cfg(not(target_os="android"))]
use std::os::raw::{c_char,c_int, c_void};

use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::any::{Any, TypeId};
use std::default::Default;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::rc::{Rc,Weak};
use std::fmt;
use std::cell::RefCell;
use std::mem::size_of;
use std::panic::{self, AssertUnwindSafe};

/*-----------------------------------------------------------------------
supported functions: V0.5.0,  test for rust version 1.25

starrust:
    InitSimple
	InitSimpleEx
	InitCore
	GetSrvGroup
	ModuleExit
	ModuleClear

	RegMsgCallBack_P     : fn(ServiceGroupID: u32, uMsg: u32, wParam: &Any, lParam: &Any) -> (bool, Box<Any>)
	RegDispatchRequest_P : fn()
	SRPDispatch
	SRPLock
	SRPUnLock
	SetRegisterCode
	IsRegistered
    SetLocale
    GetLocale
    Version
    GetScriptIndex
    SetScript
	DetachCurrentThread
	CoreHandle

StarSrvGroup:  [please refer to trait : STARSRVGROUP_TRAIT]
    CreateService
	GetService
	ClearService
    NewParaPkg   : HashMap<String,&Any>/HashMap<&str,&Any>->dictflag set,bool,...,&str,string,vec<bool>,...vec<&str>,vec<String>. does not support array,slice, and others
	NewBinBuf
	NewSXml
	IsObject
	IsParaPkg
	IsBinBuf
	IsSXml
	GetServicePath
	SetServicePath
	ServicePathIsSet
	GetCurrentPath
	ImportService
	ClearServiceEx
    RunScript
    RunScriptEx
    DoFile
    DoFileEx
	SetClientPort
    SetTelnetPort
    SetOutputPort
    SetWebServerPort
    InitRaw
    LoadRawModule
    GetLastError
    GetLastErrorInfo
	SUnLockGC
	GetCorePath
	GetUserPath
	GetLocalIP
	GetLocalIPEx
	GetObjectNum
	ActiveScriptInterface
	PreCompile

    NewParaDict  /* 2018/05/19 */

StarService:      [please refer to trait : STARSERVICE_TRAIT]
    Get          : "_Name"  "_ServiceGroup"
    GetObject
    GetObjectEx
	New
	RunScript
	RunScriptEx
	DoFile
	DoFileEx
    IsServiceRegistered
    CheckPassword
    LoadRawModule
    NewRawProxy
    ImportRawContext
    ImportRawObject
    GetLastError
    GetLastErrorInfo

StarParaPkg:    [please refer to trait : STARPARAPKG_TRAIT]
    GetNumber
    Get         : int value(index)
	GetBool
	GetInt
	GetInt64    
	GetString
	GetDouble
	GetObject
	GetParaPkg
	GetBinBuf

	Clear
	AppendFrom
	Set
	Build
	Free
	Dispose
	ReleaseOwner
	AsDict
	IsDict
	FromJSon
	ToJSon

    FromVecBool
    FromVecI8
    FromVecU8
    FromVecI16
    FromVecU16
    FromVecI32
    FromVecU32
    FromVecI64
    FromVecU64
    FromVecISize
    FromVecUSize
    FromVecF32
    FromVecF64
    FromVecString
    FromVecStr

    ToVecBool(
    ToVecInt
    ToVecInt64
    ToVecString
    ToVecDouble    

StarBinBuf :   [please refer to trait : STARBINBUF_TRAIT]
    GetOffset

	Init
	Clear
	SaveToFile
	SaveToFile
	Free
	ReleaseOwner
	Dispose

StarSXml :    [please refer to trait : STARSXML_TRAIT]
	LoadFromFile
	LoadFromBuf
	LoadFromBufEx
	SaveToFile
	SaveToBuf
	GetStandalone
	GetVersion
	GetEncoding
	FindElement
	FindElementEx
	FirstElement
	NextElement
	ParentElement
	GetElement
	GetElementEx
	GetNs
	GetNsValue
	FindAttribute
	FirstAttribute
	NextAttribute
	GetAttributeName
	GetAttributeValue
	GetSingleText
	FirstText
	NextText
	GetText
	SetDeclaration
	RemoveDeclaration

StarObject :  [please refer to trait : STAROBJECT_TRAIT]
	Get   : "_Service"  "_Class"  "_ID"  "_Name"
	GetBool
	GetInt
    GetInt64
	GetString
	GetDouble
	GetObject
	GetParaPkg
	GetBinBuf

	Set   : "_Name"
	Call
	CallBool
	CallInt
    CallInt64
	CallString
	CallDouble
	CallObject
	CallParaPkg
	CallBinBuf

	New
	Free
	Dispose
	RawToParaPkg
	DeferFree
	IsInFree
	GetSourceScript
	GetRefEx
	GetRefInfo
	IsValid
	GetLastError
	GetLastErrorInfo
	RegScriptProc_P      :  fn(CleGroup:&STARSRVGROUP,CleService:&STARSERVICE,CleObject:&STAROBJECT,Paras: &[STARRESULT]) -> STARRESULT
	ReleaseOwnerEx
	IsSLock

note:

1)  As for how to use these functions, please refer to the interface manual and programming guide

2)  Conversion of variable types :
rust                                ->       other script

bool                                               bool
i8,u8,i16,u16,i32,u32                              int32
isize,usize                                        int32 or int64
f32,f64                                            double
&str/String                                             string
HashMap<String,&Any>/HashMap<&str,&Any>            parapkg dict
vec<bool>,...vec<String>                           parapkg

3)  function can be called from other scripts
struct methods : the input or output parameter must be types defined above 
lambda functions

4)  starrust.RegScriptTermCallBack_P is used before rust share library unloaded

5)  starrust.RegScriptInitCallBack_P is used after rust share library loaded

6)  starrust.print/printw/printe/println/printlnw/printlne should be used instead of println!/print!

7)  set_env/get_env can be used to set environment variable

8)  use STARRESULT_TRAIT and STARRESULTTYPE

9)  use ToString/ToBool/ToInt/ToInt64/ToDouble/... of STARRESULT

-----------------------------------------------------------------------*/

pub type STARSRVGROUP = Rc<Option<RefCell<Option<StarSrvGroup>>>>;
pub type STARSERVICE = Rc<Option<RefCell<Option<StarService>>>>;
pub type STARPARAPKG = Rc<Option<RefCell<Option<StarParaPkg>>>>;
pub type STARBINBUF = Rc<Option<RefCell<Option<StarBinBuf>>>>;
pub type STARSXML = Rc<Option<RefCell<Option<StarSXml>>>>;
pub type STAROBJECT = Rc<Option<RefCell<Option<StarObject>>>>;

type STARSRVGROUP_WEAK = Weak<Option<RefCell<Option<StarSrvGroup>>>>;
type STARSERVICE_WEAK = Weak<Option<RefCell<Option<StarService>>>>;
type STARPARAPKG_WEAK = Weak<Option<RefCell<Option<StarParaPkg>>>>;
type STARBINBUF_WEAK = Weak<Option<RefCell<Option<StarBinBuf>>>>;
type STARSXML_WEAK = Weak<Option<RefCell<Option<StarSXml>>>>;
type STAROBJECT_WEAK = Weak<Option<RefCell<Option<StarObject>>>>;

pub type STARRESULT = Option<Box<Any>>;
pub type STARRESULT_TUPLE = Vec<STARRESULT>;

macro_rules! STARRC {
    ($name: expr) => {
        Rc::new(Some(RefCell::new($name)));
    };
}
macro_rules! STARRC_WEAK {
    () => {
        Weak::new();
    };
}

pub enum STARRESULTTYPE {
    T_BOOL,
    T_I8,
    T_U8,
    T_I16,
    T_U16,
    T_I32,
    T_U32,
    T_I64,
    T_U64,
    T_ISIZE,
    T_USIZE,
    T_F32,
    T_F64,
    T_STRING,
    T_STAROBJECT,
    T_STARBINBUF,
    T_STARPARAPKG,
    T_STARSXML,
    T_STARSERVICE,
    T_STARSRVGROUP,        
    T_STARRESULT_TUPLE,
    T_NONE,
}

impl fmt::Debug for STARRESULTTYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s : &str;
        match self {
        &STARRESULTTYPE::T_BOOL => s = "bool",
        &STARRESULTTYPE::T_I8 => s = "i8",
        &STARRESULTTYPE::T_U8 => s = "u8",
        &STARRESULTTYPE::T_I16 => s = "i16",
        &STARRESULTTYPE::T_U16 => s = "u16",
        &STARRESULTTYPE::T_I32 => s = "i32",
        &STARRESULTTYPE::T_U32 => s = "u32",
        &STARRESULTTYPE::T_I64 => s = "i64",
        &STARRESULTTYPE::T_U64 => s = "u64",
        &STARRESULTTYPE::T_ISIZE => s = "isize",
        &STARRESULTTYPE::T_USIZE => s = "usize",
        &STARRESULTTYPE::T_F32 => s = "f32",
        &STARRESULTTYPE::T_F64 => s = "f64",
        &STARRESULTTYPE::T_STRING => s = "String", 

        &STARRESULTTYPE::T_STAROBJECT => s = "STAROBJECT",
        &STARRESULTTYPE::T_STARBINBUF => s = "STARBINBUF",
        &STARRESULTTYPE::T_STARPARAPKG => s = "STARPARAPKG",
        &STARRESULTTYPE::T_STARSXML => s = "STARSXML",
        &STARRESULTTYPE::T_STARSERVICE => s = "STARSERVICE",
        &STARRESULTTYPE::T_STARSRVGROUP => s = "STARSRVGROUP",                      
        &STARRESULTTYPE::T_STARRESULT_TUPLE => s = "STARRESULT_TUPLE",

        _ => s = "NONE",
        }
        write!(f, "{}",s)
    }
}


pub trait STARRESULT_TRAIT {
    fn Type(&self) -> STARRESULTTYPE;  
    fn ToBool(&self) -> bool;
    fn ToInt(&self) -> i32;
    fn ToInt64(&self) -> i64;
    fn ToString(&self) -> String;
    fn ToDouble(&self) -> f64;
    fn ToObject(&self)->STAROBJECT;
    fn ToParaPkg(&self) -> STARPARAPKG;
    fn ToBinBuf(&self) -> STARBINBUF;  
    fn ToSXml(&self) -> STARSXML;     
    fn ToService(&self) -> STARSERVICE; 
    fn ToSrvGroup(&self) -> STARSRVGROUP;         
    fn ToTuple<'a>(&'a self) -> &'a STARRESULT_TUPLE;  
}

impl STARRESULT_TRAIT for STARRESULT {
    fn Type(&self) -> STARRESULTTYPE{
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if vv.is::<bool>() {
                return STARRESULTTYPE::T_BOOL;
            }else if vv.is::<i8>() {
                return STARRESULTTYPE::T_I8;
            }else if vv.is::<u8>() {
                return STARRESULTTYPE::T_U8;
            }else if vv.is::<i16>() {
                return STARRESULTTYPE::T_I16;
            }else if vv.is::<u16>() {
                return STARRESULTTYPE::T_U16;
            }else if vv.is::<i32>() {
                return STARRESULTTYPE::T_I32;
            }else if vv.is::<u32>() {
                return STARRESULTTYPE::T_U32;
            }else if vv.is::<i64>() {
                return STARRESULTTYPE::T_I64;
            }else if vv.is::<u64>() {
                return STARRESULTTYPE::T_U64;
            }else if vv.is::<isize>() {
                return STARRESULTTYPE::T_ISIZE;
            }else if vv.is::<usize>() {
                return STARRESULTTYPE::T_USIZE;
            }else if vv.is::<f32>() {
                return STARRESULTTYPE::T_F32;                                                                                                                                                                
            }else if vv.is::<f64>() {
                return STARRESULTTYPE::T_F64;     
            }else if vv.is::<String>() {
                return STARRESULTTYPE::T_STRING; 
            }else if vv.is::<&str>() {
                return STARRESULTTYPE::T_STRING;                                                                                                                                                                                            
            }else if vv.is::<STAROBJECT>() {
                return STARRESULTTYPE::T_STAROBJECT;                                                                                                                                                                
            }else if vv.is::<STARBINBUF>() {
                return STARRESULTTYPE::T_STARBINBUF;                                                                                                                                                                
            }else if vv.is::<STARPARAPKG>() {
                return STARRESULTTYPE::T_STARPARAPKG;                                                                                                                                                                
            }else if vv.is::<STARSXML>() {
                return STARRESULTTYPE::T_STARSXML;  
            }else if vv.is::<STARSERVICE>() {
                return STARRESULTTYPE::T_STARSERVICE;  
            }else if vv.is::<STARSRVGROUP>() {
                return STARRESULTTYPE::T_STARSRVGROUP;                                                                                                                                                                                                                   
            }else if vv.is::<STARRESULT_TUPLE>() {
                return STARRESULTTYPE::T_STARRESULT_TUPLE;                                                                                                                                                                
                
            }else{
                return STARRESULTTYPE::T_NONE;
            }
        }else{
            return STARRESULTTYPE::T_NONE;
        }
    }

    fn ToBool(&self) -> bool {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<bool>() {
                return *fdata;
            }else{
                return false;
            }
        }else{
            return false;
        }
    }

    fn ToInt(&self) -> i32 {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToInt(vv,true) as i32;
        }else{
            return 0;
        }
    }

    fn ToInt64(&self) -> i64 {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToInt64(vv,true) as i64;
        }else{
            return 0;
        }
    }

    fn ToString(&self) -> String {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<String>() {
                return fdata.clone();
            }else if let Some(fdata) = vv.downcast_ref::<&str>() {
                return fdata.to_string();
            }else{
                return "".to_owned();
            }
        }else{
            return "".to_owned();
        }
    }

    fn ToDouble(&self) -> f64 {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToFloat(vv) as f64;
        }else{
            return 0.0;
        }
    }

    fn ToObject(&self)->STAROBJECT {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STAROBJECT>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn ToParaPkg(&self) -> STARPARAPKG {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARPARAPKG>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn ToBinBuf(&self) -> STARBINBUF {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARBINBUF>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }    

    fn ToSXml(&self) -> STARSXML {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARSXML>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }   

    fn ToService(&self) -> STARSERVICE {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARSERVICE>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }       

    fn ToSrvGroup(&self) -> STARSRVGROUP {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARSRVGROUP>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }         

    fn ToTuple<'a>(&'a self) -> &'a STARRESULT_TUPLE {
        if let Some(v) = self.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARRESULT_TUPLE>() {
                return fdata;
            }else{
                panic!("not STARRESULT_TUPLE");
            }
        }else{
            panic!("not STARRESULT_TUPLE");
        }
    }    
}

/*--this function should not be called outside of the caret--*/
fn STARGETRCMUT_STARSRVGROUP_ToRef<'a>(val: &'a Option<RefCell<Option<StarSrvGroup>>>) -> &'a RefCell<Option<StarSrvGroup>> 
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
}    

fn STARGETRCMUT_STARSRVGROUP_ToMut<'a>(val: &'a Option<RefCell<Option<StarSrvGroup>>>) -> &'a RefCell<Option<StarSrvGroup>>
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
} 

macro_rules! STARGETRCMUT_STARSRVGROUP {
    ($name: expr) => {
        STARGETRCMUT_STARSRVGROUP_ToMut($name.as_ref()).borrow_mut() /*.as_mut()*/
    };
}
macro_rules! STARGETRCREF_STARSRVGROUP {
    ($name: expr) => {
        STARGETRCMUT_STARSRVGROUP_ToRef($name.as_ref()).borrow().as_ref()
    };
}

//++++++++++++++++++
fn STARGETRCMUT_STARSERVICE_ToRef<'a>(val: &'a Option<RefCell<Option<StarService>>>) -> &'a RefCell<Option<StarService>> 
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
}    

fn STARGETRCMUT_STARSERVICE_ToMut<'a>(val: &'a Option<RefCell<Option<StarService>>>) -> &'a RefCell<Option<StarService>>
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
} 

macro_rules! STARGETRCMUT_STARSERVICE {
    ($name: expr) => {
        STARGETRCMUT_STARSERVICE_ToMut($name.as_ref()).borrow_mut() /*.as_mut()*/
    };
}
macro_rules! STARGETRCREF_STARSERVICE {
    ($name: expr) => {
        STARGETRCMUT_STARSERVICE_ToRef($name.as_ref()).borrow().as_ref()
    };
}

//++++++++++++++++++
fn STARGETRCMUT_STARPARAPKG_ToRef<'a>(val: &'a Option<RefCell<Option<StarParaPkg>>>) -> &'a RefCell<Option<StarParaPkg>> 
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
}    

fn STARGETRCMUT_STARPARAPKG_ToMut<'a>(val: &'a Option<RefCell<Option<StarParaPkg>>>) -> &'a RefCell<Option<StarParaPkg>>
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
} 

macro_rules! STARGETRCMUT_STARPARAPKG {
    ($name: expr) => {
        STARGETRCMUT_STARPARAPKG_ToMut($name.as_ref()).borrow_mut() /*.as_mut()*/
    };
}
macro_rules! STARGETRCREF_STARPARAPKG {
    ($name: expr) => {
        STARGETRCMUT_STARPARAPKG_ToRef($name.as_ref()).borrow().as_ref()
    };
}

//++++++++++++++++++
fn STARGETRCMUT_STARBINBUF_ToRef<'a>(val: &'a Option<RefCell<Option<StarBinBuf>>>) -> &'a RefCell<Option<StarBinBuf>> 
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
}    

fn STARGETRCMUT_STARBINBUF_ToMut<'a>(val: &'a Option<RefCell<Option<StarBinBuf>>>) -> &'a RefCell<Option<StarBinBuf>>
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
} 

macro_rules! STARGETRCMUT_STARBINBUF {
    ($name: expr) => {
        STARGETRCMUT_STARBINBUF_ToMut($name.as_ref()).borrow_mut() /*.as_mut()*/
    };
}
macro_rules! STARGETRCREF_STARBINBUF {
    ($name: expr) => {
        STARGETRCMUT_STARBINBUF_ToRef($name.as_ref()).borrow().as_ref()
    };
}

//++++++++++++++++++
fn STARGETRCMUT_STARSXML_ToRef<'a>(val: &'a Option<RefCell<Option<StarSXml>>>) -> &'a RefCell<Option<StarSXml>> 
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
}    

fn STARGETRCMUT_STARSXML_ToMut<'a>(val: &'a Option<RefCell<Option<StarSXml>>>) -> &'a RefCell<Option<StarSXml>>
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
} 

macro_rules! STARGETRCMUT_STARSXML {
    ($name: expr) => {
        STARGETRCMUT_STARSXML_ToMut($name.as_ref()).borrow_mut() /*.as_mut()*/
    };
}
macro_rules! STARGETRCREF_STARSXML {
    ($name: expr) => {
        STARGETRCMUT_STARSXML_ToRef($name.as_ref()).borrow().as_ref()
    };
}

//++++++++++++++++++
fn STARGETRCMUT_STAROBJECT_ToRef<'a>(val: &'a Option<RefCell<Option<StarObject>>>) -> &'a RefCell<Option<StarObject>> 
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
}    

fn STARGETRCMUT_STAROBJECT_ToMut<'a>(val: &'a Option<RefCell<Option<StarObject>>>) -> &'a RefCell<Option<StarObject>>
{
    if let Some(v) = val.as_ref() {
        return v;
    }else{
        panic!("error STARGETRCMUT_ToRef");
    }
} 

macro_rules! STARGETRCMUT_STAROBJECT {
    ($name: expr) => {
        STARGETRCMUT_STAROBJECT_ToMut($name.as_ref()).borrow_mut() /*.as_mut()*/
    };
}
macro_rules! STARGETRCREF_STAROBJECT {
    ($name: expr) => {
        STARGETRCMUT_STAROBJECT_ToRef($name.as_ref()).borrow().as_ref()
    };
}

//-------------------------------------------------------------------------------------------------
fn STARSRVGROUP_IsValid<'a>(val: &'a STARSRVGROUP) -> bool {
    if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(val) {
        return ObjData.IsValid;
    } else {
        return false;
    }
}

fn STARSRVGROUP_RefItem<'a>(val: &'a STARSRVGROUP) -> usize {
    if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(val) {
        return ObjData.ObjData.RefItem;
    } else {
        return 0;
    }
}

fn STARSERVICE_IsValid<'a>(val: &'a STARSERVICE) -> bool {
    if let Some(ObjData) = STARGETRCREF_STARSERVICE!(val) {
        return ObjData.IsValid;
    } else {
        return false;
    }
}
fn STARSERVICE_RefItem<'a>(val: &'a STARSERVICE) -> usize {
    if let Some(ObjData) = STARGETRCREF_STARSERVICE!(val) {
        return ObjData.ObjData.RefItem;
    } else {
        return 0;
    }
}

fn STARPARAPKG_IsValid<'a>(val: &'a STARPARAPKG) -> bool {
    if let Some(ObjData) = STARGETRCREF_STARPARAPKG!(val) {
        return ObjData.IsValid;
    } else {
        return false;
    }
}
fn STARPARAPKG_RefItem<'a>(val: &'a STARPARAPKG) -> usize {
    if let Some(ObjData) = STARGETRCREF_STARPARAPKG!(val) {
        return ObjData.ObjData.RefItem;
    } else {
        return 0;
    }
}

fn STARBINBUF_IsValid<'a>(val: &'a STARBINBUF) -> bool {
    if let Some(ObjData) = STARGETRCREF_STARBINBUF!(val) {
        return ObjData.IsValid;
    } else {
        return false;
    }
}
fn STARBINBUF_RefItem<'a>(val: &'a STARBINBUF) -> usize {
    if let Some(ObjData) = STARGETRCREF_STARBINBUF!(val) {
        return ObjData.ObjData.RefItem;
    } else {
        return 0;
    }
}

fn STARSXML_IsValid<'a>(val: &'a STARSXML) -> bool {
    if let Some(ObjData) = STARGETRCREF_STARSXML!(val) {
        return ObjData.IsValid;
    } else {
        return false;
    }
}
fn STARSXML_RefItem<'a>(val: &'a STARSXML) -> usize {
    if let Some(ObjData) = STARGETRCREF_STARSXML!(val) {
        return ObjData.ObjData.RefItem;
    } else {
        return 0;
    }
}

fn STAROBJECT_IsValid<'a>(val: &'a STAROBJECT) -> bool {
    if let Some(ObjData) = STARGETRCREF_STAROBJECT!(val) {
        return ObjData.IsValid;
    } else {
        return false;
    }
}
fn STAROBJECT_RefItem<'a>(val: &'a STAROBJECT) -> usize {
    if let Some(ObjData) = STARGETRCREF_STAROBJECT!(val) {
        return ObjData.ObjData.RefItem;
    } else {
        return 0;
    }
}

//------------------------------------------------------------------------------

#[repr(C)]
pub struct VS_QUERYRECORD {
    pub Reserved: [i8; 32],
}
impl VS_QUERYRECORD {
    pub fn new() -> VS_QUERYRECORD {
        VS_QUERYRECORD {
            Reserved: unsafe { mem::zeroed() },
        }
    }
}

#[repr(C)]
pub struct VSSOCKADDR_IN {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [c_char; 8],
}
impl VSSOCKADDR_IN {
    pub fn new() -> VSSOCKADDR_IN {
        VSSOCKADDR_IN {
            sin_family: 0 as u16,
            sin_port: 0 as u16,
            sin_addr: 0 as u32,
            sin_zero: unsafe { mem::zeroed() },
        }
    }
}


#[repr(C)]
pub struct VS_UUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [i8; 8],
}
impl VS_UUID {
    pub fn new() -> VS_UUID {
        VS_UUID {
            Data1: 0 as u32,
            Data2: 0 as u16,
            Data3: 0 as u16,
            Data4: unsafe { mem::zeroed() },
        }
    }
}

impl Clone for VS_UUID {
    fn clone(&self) -> Self {
        let mut aaa = VS_UUID::new();
        aaa.Data1 = self.Data1;
        aaa.Data2 = self.Data2;
        aaa.Data3 = self.Data3;
        aaa.Data4 = self.Data4;
        return aaa;
    }
}

#[repr(C)]
struct VS_ATTRIBUTEINFO{
    Name : [c_char;40],
    Caption : [c_char;40],
    DefaultString : [c_char;40],
    Type : u8,
    EditType : u8,
    EditReadOnly : u8,
    SyncType : VS_BOOL, 
    CreateNeedFlag : u8,
    ChangeNotifyFlag : u8,
    Length : i32,
    Offset : i32,
    ComboBoxID : [u8;20],
    StructID : VS_UUID,
    StaticID : u32,
    AtomicAttributeIndex : u8,
    AttributeIndex : u8,
    Reserved : [u8;2],
    AtomicAttributeObject : *const c_void,
}

impl VS_ATTRIBUTEINFO {
    pub fn new() -> VS_ATTRIBUTEINFO {
        VS_ATTRIBUTEINFO {
            Name : unsafe { mem::zeroed() },
            Caption : unsafe { mem::zeroed() },
            DefaultString : unsafe { mem::zeroed() },
            Type : 0u8,
            EditType : 0u8,
            EditReadOnly : 0u8,
            SyncType : VS_FALSE, 
            CreateNeedFlag : 0u8,
            ChangeNotifyFlag : 0u8,
            Length : 0i32,
            Offset : 0i32,
            ComboBoxID : unsafe { mem::zeroed() }, 
            StructID : VS_UUID::new(),
            StaticID : 0u32,
            AtomicAttributeIndex : 0u8,
            AttributeIndex : 0u8,
            Reserved : unsafe { mem::zeroed() },
            AtomicAttributeObject : 0 as *const c_void,
        }
    }
}

#[repr(C)]
struct VS_VSTRING{
    Buf : *mut c_char,
}

enum StructOfSRPComm_HttpOnRequest {}
enum VSPublicServiceDef {}
enum StructOfVSScriptContext {}
enum StructOfVSServerUrlInfo {}
enum VS_EVENTPARAM_RUNPARAM {}
enum VS_STATISTICINFO {}
enum VS_UPDOWNFILEINFO {}
enum VSWINDOWSTYLE {}
enum VSWINDOWPOSITION {}
enum StructOfVSRunEnv {}
enum VS_TIME {}
enum VS_SERVICEINFO {}
//enum VS_VSTRING {}
//enum VS_ATTRIBUTEINFO {}
enum VS_COMBOBOXITEM {}
enum VS_FUNCTIONINFO {}
enum VS_OUTEVENTINFO {}
enum VS_EVENTPARAM {}
enum VS_ACTIVESETITEM {}
enum VS_CLIENTINFO {}
enum VS_CLIENTQOS {}
enum VS_UPDOWNFILEMSG {}
enum VS_RECT {}
enum VS_FONT {}
enum VSLuaL_Reg {}
enum VSImportServiceDef {}
//enum VS_UUID{}
enum VSWindowlessSiteCallBackInfo {}

type VS_BOOL = i8;

include!("vsopenapi_c_stub.rs");
include!("starrust_native.rs");
include!("starrust_def.rs");

/*---------------------constant---------------------*/
pub static MSG_APPEVENT: u32 = 0x00000028;
pub static MSG_VSDISPMSG: u32 = 0x00000001;
pub static MSG_VSDISPLUAMSG: u32 = 0x00000002;
pub static MSG_DISPMSG: u32 = 0x00000003;
pub static MSG_DISPLUAMSG: u32 = 0x00000004;
pub static MSG_EXIT: u32 = 0x00000006;
pub static MSG_HYPERLINK: u32 = 0x00000026;
pub static MSG_SETMANAGERCAPTION: u32 = 0x00000033;
pub static MSG_ONTELNETSTRING_PREEXECUTE: u32 = 0x0000007B;
pub static MSG_ONTELNETSTRING: u32 = 0x0000007A;

static OBJECTTYPE_STARSRVGROUP: u32 = 1;
static OBJECTTYPE_STARSERVICE: u32 = 2;
static OBJECTTYPE_STARSERVICEITEM: u32 = 3;
static OBJECTTYPE_STARPARAPKG: u32 = 4;
static OBJECTTYPE_STARQUERYRECORD: u32 = 5;
static OBJECTTYPE_STARBINBUF: u32 = 6;
static OBJECTTYPE_STARSXML: u32 = 7;
static OBJECTTYPE_STARFUNCTIONPARA: u32 = 8;
static OBJECTTYPE_STARCOMMINTERFACE: u32 = 9;
static OBJECTTYPE_STAROBJECT: u32 = 10;
static OBJECTTYPE_STARSTRUCT: u32 = 11;
static OBJECTTYPE_STARFUNCTION: u32 = 12;
static OBJECTTYPE_STARCALLBACK: u32 = 13;
static OBJECTTYPE_INTERFACE: u32 = 15;

static VSFAULT_INDICATION: i32 = 0x00;
static VSFAULT_WARNING: i32 = 0x01;
static VSFAULT_NORMALERROR: i32 = 0x02;
static VSFAULT_CRITICALERROR: i32 = 0x03;
static VSFAULT_SYSTEMERROR: i32 = 0x04;
static VSFAULT_DISP: i32 = 0x06;
static VSFAULT_OPENSHOW: i32 = 0x07;

static VS_INVALID_SERVICEGROUPID: u32 = 0xFFFFFFFF;

static VS_TRUE: VS_BOOL = 1;
static VS_FALSE: VS_BOOL = 0;

const VSTYPE_PTR: u8 = 14;
const VSTYPE_BOOL: u8 = 1;
const VSTYPE_INT8: u8 = 2;
const VSTYPE_UINT8: u8 = 3;
const VSTYPE_INT16: u8 = 4;
const VSTYPE_UINT16: u8 = 5;
const VSTYPE_INT32: u8 = 6;
const VSTYPE_UINT32: u8 = 7;
const VSTYPE_FLOAT: u8 = 8;
const VSTYPE_LONG: u8 = 9;
const VSTYPE_ULONG: u8 = 10;
const VSTYPE_LONGHEX: u8 = 11;
const VSTYPE_ULONGHEX: u8 = 12;
const VSTYPE_VSTRING: u8 = 51;
const VSTYPE_UUID: u8 = 41;
const VSTYPE_STATICID: u8 = 29;
const VSTYPE_CHARPTR: u8 = 30;
const VSTYPE_STRUCT: u8 = 16;
const VSTYPE_COLOR: u8 = 19;
const VSTYPE_INT64: u8 = 60;
const VSTYPE_DOUBLE : u8 = 58;
const VSTYPE_CHAR : u8 = 13;

const SRPPARATYPE_INVALID : i32 = 0;
const SRPPARATYPE_INT : i32 = 1;
const SRPPARATYPE_FLOAT : i32 = 2;
const SRPPARATYPE_BIN : i32 = 3;
const SRPPARATYPE_CHARPTR : i32 = 4;
const SRPPARATYPE_TIME : i32 = 5;
const SRPPARATYPE_BOOL : i32 = 6;
const SRPPARATYPE_OBJECT : i32 = 7;
const SRPPARATYPE_PARAPKG : i32 = 8;
const SRPPARATYPE_INT64 : i32 = 9;

const VSLUATYPE_INT : i32 = 16;
const VSLUATYPE_INT64 : i32 = 19; 
const VSLUATYPE_UWORD : i32 = 20;
const VSLUATYPE_NUMBER : i32 = 1;
const VSLUATYPE_BOOL : i32 = 2;
const VSLUATYPE_STRING : i32 = 3;
const VSLUATYPE_FUNCTION : i32 = 4;
const VSLUATYPE_TABLE : i32 = 5;
const VSLUATYPE_OBJECT : i32 = 6;
const VSLUATYPE_PARAPKG  : i32 = 7;
const VSLUATYPE_USERDATA : i32 = 17 ;
const VSLUATYPE_LIGHTUSERDATA : i32 = 18;
const VSLUATYPE_BINBUF : i32 = 12;
const VSLUATYPE_SXML : i32 = 13;
const VSLUATYPE_NIL : i32 =  0;

const M_NULL: *mut c_void = 0 as *mut c_void;
const NULL: *const c_void = 0 as *const c_void;

fn IsNULL(Val: *const c_void) -> bool {
    if Val == (0 as *const c_void) {
        return true;
    }
    return false;
}

fn IsM_NULL(Val: *mut c_void) -> bool {
    if Val == (0 as *mut c_void) {
        return true;
    }
    return false;
}

/*-----------------------------------------------------------------------*/
/*--global variable*/
/*-----------------------------------------------------------------------*/
fn Default_MsgCallBack(
    ServiceGroupID: u32,
    uMsg: u32,
    wParam: &Any,
    lParam: &Any,
) -> (bool, Box<Any>) {
    return (false, Box::new(0));
}
fn Default_DispatchRequestCallBack() {}
fn Default_ScriptTermCallBack() {}
fn Default_ScriptInitCallBack(SrvGroup: &STARSRVGROUP, Service: &STARSERVICE) {}

static mut G_MSGCALLBACK: fn(ServiceGroupID: u32, uMsg: u32, wParam: &Any, lParam: &Any)
    -> (bool, Box<Any>) = Default_MsgCallBack;
static mut G_DISPATCHREQUESTCALLBACK: fn() = Default_DispatchRequestCallBack;
pub static mut G_SCRIPTTERMCALLBACK: fn() = Default_ScriptTermCallBack;
pub static mut G_SCRIPTINITCALLBACK: fn(SrvGroup: &STARSRVGROUP, Service: &STARSERVICE) = Default_ScriptInitCallBack;

extern {
    fn ScriptTermCallBack();
    fn ScriptInitCallBack(SrvGroup: &STARSRVGROUP, Service: &STARSERVICE);
}    

/*-----------------------------------------------------------------------*/
/*--weak reference table                                                 */
/*-----------------------------------------------------------------------*/

struct _StarCore_WeakTableItem {
    RefCount: usize,
    ObjType: u32,

    ObjData_StarSrvGroup: STARSRVGROUP_WEAK, //weak reference
    ObjData_StarService: STARSERVICE_WEAK,   //weak reference
    ObjData_StarParaPkg: STARPARAPKG_WEAK,   //weak reference
    ObjData_StarBinBuf: STARBINBUF_WEAK,     //weak reference
    ObjData_StarSXml: STARSXML_WEAK,         //weak reference
    ObjData_StarObject: STAROBJECT_WEAK,     //weak reference    
    
    SObjData_StarSrvGroup: STARSRVGROUP, //strong reference
    SObjData_StarService: STARSERVICE,   //strong reference
    SObjData_StarParaPkg: STARPARAPKG,   //strong reference
    SObjData_StarBinBuf: STARBINBUF,     //strong reference
    SObjData_StarSXml: STARSXML,         //strong reference
    SObjData_StarObject: STAROBJECT,     //strong reference
}
impl _StarCore_WeakTableItem {
    pub fn new() -> _StarCore_WeakTableItem {
        _StarCore_WeakTableItem {
            RefCount: 0 as usize,
            ObjType: 0 as u32,

            ObjData_StarSrvGroup: STARRC_WEAK!(),
            ObjData_StarService: STARRC_WEAK!(),
            ObjData_StarParaPkg: STARRC_WEAK!(),
            ObjData_StarBinBuf: STARRC_WEAK!(),
            ObjData_StarSXml: STARRC_WEAK!(),
            ObjData_StarObject: STARRC_WEAK!(),

            SObjData_StarSrvGroup: STARRC!(None),
            SObjData_StarService: STARRC!(None),
            SObjData_StarParaPkg: STARRC!(None),
            SObjData_StarBinBuf: STARRC!(None),
            SObjData_StarSXml: STARRC!(None),
            SObjData_StarObject: STARRC!(None),
        }
    }
}

static mut G_WEAKTABLE_REFCOUNT: usize = 1;
static mut G_WEAKTABLE: *mut HashMap<usize, _StarCore_WeakTableItem> = 0 as *mut _; /*rust does not support global variable*/
static mut WEAKTABLEMUTEXINITFLAG: bool = false;
static mut WEAKTABLEMUTEX: *mut c_void = 0 as *mut _;

/*---create new weak reference--*/
fn NewRustObjectRef_StarSrvGroup(In_ObjData: &STARSRVGROUP, RefCount: usize) -> usize {
    unsafe {
        let mut STARGETRCMUT_Temp = STARGETRCMUT_STARSRVGROUP!(In_ObjData);
        if let Some(ObjData) = STARGETRCMUT_Temp.as_mut() {
            starrust_mutex_lock(WEAKTABLEMUTEX);
            if G_WEAKTABLE == 0 as *mut _ {
                G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
            }
            let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
            if RefCount != 0 {
                let ok = l_WeakTable.contains_key(&RefCount);
                if ok {
                    let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                }
            }
            let mut item = _StarCore_WeakTableItem::new();
            item.RefCount = 0;
            item.ObjType = OBJECTTYPE_STARSRVGROUP;
            item.ObjData_StarSrvGroup = Rc::downgrade(In_ObjData);
            if RefCount == 0 {
                l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
            }else{
                l_WeakTable.insert(RefCount, item);
            }            
            let NewRefCount : usize; 
            if RefCount == 0 {
                NewRefCount = G_WEAKTABLE_REFCOUNT;
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
                G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
            }else{
                NewRefCount = RefCount;
                ObjData.ObjData.RefItem = RefCount;                    
            }
            starrust_mutex_unlock(WEAKTABLEMUTEX);
            return NewRefCount;
        } else {
            return 0;
        }
    }
}

fn NewRustObjectRef_StarService(In_ObjData: &STARSERVICE, RefCount: usize) -> usize {
    unsafe {
        let mut STARGETRCMUT_Temp = STARGETRCMUT_STARSERVICE!(In_ObjData);
        if let Some(ObjData) = STARGETRCMUT_Temp.as_mut() {
            starrust_mutex_lock(WEAKTABLEMUTEX);
            if G_WEAKTABLE == 0 as *mut _ {
                G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
            }
            let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
            if RefCount != 0 {
                let ok = l_WeakTable.contains_key(&RefCount);
                if ok {
                    let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                }
            }
            let mut item = _StarCore_WeakTableItem::new();
            item.RefCount = 0;
            item.ObjType = OBJECTTYPE_STARSERVICE;
            item.ObjData_StarService = Rc::downgrade(In_ObjData);
            if RefCount == 0 {
                l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
            }else{
                l_WeakTable.insert(RefCount, item);
            }            
            let NewRefCount : usize; 
            if RefCount == 0 {
                NewRefCount = G_WEAKTABLE_REFCOUNT;
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
                G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
            }else{
                NewRefCount = RefCount;
                ObjData.ObjData.RefItem = RefCount;                    
            }
            starrust_mutex_unlock(WEAKTABLEMUTEX);
            return NewRefCount;
        } else {
            return 0;
        }
    }
}

fn NewRustObjectRef_StarParaPkg(In_ObjData: &STARPARAPKG, RefCount: usize) -> usize {
    unsafe {
        let mut STARGETRCMUT_Temp = STARGETRCMUT_STARPARAPKG!(In_ObjData);
        if let Some(ObjData) = STARGETRCMUT_Temp.as_mut() {
            starrust_mutex_lock(WEAKTABLEMUTEX);
            if G_WEAKTABLE == 0 as *mut _ {
                G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
            }
            let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
            if RefCount != 0 {
                let ok = l_WeakTable.contains_key(&RefCount);
                if ok {
                    let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                }
            }
            let mut item = _StarCore_WeakTableItem::new();
            item.RefCount = 0;
            item.ObjType = OBJECTTYPE_STARPARAPKG;
            item.ObjData_StarParaPkg = Rc::downgrade(In_ObjData);
            if RefCount == 0 {
                l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
            }else{
                l_WeakTable.insert(RefCount, item);
            }            
            let NewRefCount : usize; 
            if RefCount == 0 {
                NewRefCount = G_WEAKTABLE_REFCOUNT;
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
                G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
            }else{
                NewRefCount = RefCount;
                ObjData.ObjData.RefItem = RefCount;                    
            }
            starrust_mutex_unlock(WEAKTABLEMUTEX);
            return NewRefCount;
        } else {
            return 0;
        }
    }
}

fn NewRustObjectRef_StarBinBuf(In_ObjData: &STARBINBUF, RefCount: usize) -> usize {
    unsafe {
        let mut STARGETRCMUT_Temp = STARGETRCMUT_STARBINBUF!(In_ObjData);
        if let Some(ObjData) = STARGETRCMUT_Temp.as_mut() {
            starrust_mutex_lock(WEAKTABLEMUTEX);
            if G_WEAKTABLE == 0 as *mut _ {
                G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
            }
            let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
            if RefCount != 0 {
                let ok = l_WeakTable.contains_key(&RefCount);
                if ok {
                    let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                }
            }
            let mut item = _StarCore_WeakTableItem::new();
            item.RefCount = 0;
            item.ObjType = OBJECTTYPE_STARBINBUF;
            item.ObjData_StarBinBuf = Rc::downgrade(In_ObjData);
            if RefCount == 0 {
                l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
            }else{
                l_WeakTable.insert(RefCount, item);
            }            
            let NewRefCount : usize; 
            if RefCount == 0 {
                NewRefCount = G_WEAKTABLE_REFCOUNT;
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
                G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
            }else{
                NewRefCount = RefCount;
                ObjData.ObjData.RefItem = RefCount;                    
            }
            starrust_mutex_unlock(WEAKTABLEMUTEX);
            return NewRefCount;
        } else {
            return 0;
        }
    }
}

fn NewRustObjectRef_StarSXml(In_ObjData: &STARSXML, RefCount: usize) -> usize {
    unsafe {
        let mut STARGETRCMUT_Temp = STARGETRCMUT_STARSXML!(In_ObjData);
        if let Some(ObjData) = STARGETRCMUT_Temp.as_mut() {
            starrust_mutex_lock(WEAKTABLEMUTEX);
            if G_WEAKTABLE == 0 as *mut _ {
                G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
            }
            let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
            if RefCount != 0 {
                let ok = l_WeakTable.contains_key(&RefCount);
                if ok {
                    let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                }
            }
            let mut item = _StarCore_WeakTableItem::new();
            item.RefCount = 0;
            item.ObjType = OBJECTTYPE_STARSXML;
            item.ObjData_StarSXml= Rc::downgrade(In_ObjData);
            if RefCount == 0 {
                l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
            }else{
                l_WeakTable.insert(RefCount, item);
            }            
            let NewRefCount : usize; 
            if RefCount == 0 {
                NewRefCount = G_WEAKTABLE_REFCOUNT;
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
                G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
            }else{
                NewRefCount = RefCount;
                ObjData.ObjData.RefItem = RefCount;                    
            }
            starrust_mutex_unlock(WEAKTABLEMUTEX);
            return NewRefCount;
        } else {
            return 0;
        }
    }
}

fn NewRustObjectRef_StarObject(In_ObjData: &STAROBJECT, RefCount: usize) -> usize {
    unsafe {
        let mut STARGETRCMUT_Temp = STARGETRCMUT_STAROBJECT!(In_ObjData);
        if let Some(ObjData) = STARGETRCMUT_Temp.as_mut() {
            starrust_mutex_lock(WEAKTABLEMUTEX);
            if G_WEAKTABLE == 0 as *mut _ {
                G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
            }
            let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
            if RefCount != 0 {
                let ok = l_WeakTable.contains_key(&RefCount);
                if ok {
                    let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                }
            }
            let mut item = _StarCore_WeakTableItem::new();
            item.RefCount = 0;
            item.ObjType = OBJECTTYPE_STAROBJECT;
            item.ObjData_StarObject = Rc::downgrade(In_ObjData);
            if RefCount == 0 {
                l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
            }else{
                l_WeakTable.insert(RefCount, item);
            }
            let NewRefCount : usize; 
            if RefCount == 0 {
                NewRefCount = G_WEAKTABLE_REFCOUNT;
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
                G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
            }else{
                NewRefCount = RefCount;
                ObjData.ObjData.RefItem = RefCount;                    
            }
            starrust_mutex_unlock(WEAKTABLEMUTEX);
            return NewRefCount;
        } else {
            return 0;
        }
    }
}

/*---input is raw pointer---*/
/*  usage:
    let s_refval = NewRustObjectRefStrong_StarSrvGroup(&Rc<StarSrvGroup>, 0);            
*/

fn NewRustObjectRefStrong_StarSrvGroup(In_ObjData: &STARSRVGROUP, RefCount: usize) -> usize {
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarSrvGroup.upgrade();
                if sss.is_some() {
                    //olditem.ObjData_StarSrvGroup = Rc::downgrade(&sss.unwrap());
                    //panic!("strong reference type error, it holds weak ref")
                }else if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(olditem.SObjData_StarSrvGroup) {
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                } else {
                    panic!("strong reference type error")
                }
            }
        }
        let mut item = _StarCore_WeakTableItem::new();
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(In_ObjData); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            if RefCount == 0 {
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
            }else{
                ObjData.ObjData.RefItem = RefCount;
            }
        }
        item.RefCount = 0 as usize;
        item.ObjType = OBJECTTYPE_STARSRVGROUP;
        item.ObjData_StarSrvGroup = STARRC_WEAK!();
        item.SObjData_StarSrvGroup = In_ObjData.clone();
        /*--if old exist, replace old--*/
        if RefCount == 0 { 
            l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
        }else{
            l_WeakTable.insert(RefCount, item);
        }        
        let NewRefCount : usize; 
        if RefCount == 0 {
            NewRefCount = G_WEAKTABLE_REFCOUNT;
            G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
        }else{
            NewRefCount = RefCount;
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return NewRefCount;
    }
}

/*---input is raw pointer---*/
/*  usage:
    let s_refval = NewRustObjectRefStrong_StarService(&Rc<StarService>, 0);            
*/

fn NewRustObjectRefStrong_StarService(In_ObjData: &STARSERVICE, RefCount: usize) -> usize {
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarService.upgrade();
                if sss.is_some() {
                    //olditem.ObjData_StarService = Rc::downgrade(&sss.unwrap());
                    //panic!("strong reference type error, it holds weak ref")
                }else if let Some(ObjData) = STARGETRCREF_STARSERVICE!(olditem.SObjData_StarService) {
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                } else {
                    panic!("strong reference type error")
                }
            }
        }
        let mut item = _StarCore_WeakTableItem::new();
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSERVICE!(In_ObjData);
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            if RefCount == 0 {
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
            }else{
                ObjData.ObjData.RefItem = RefCount;
            }
        }
        item.RefCount = 0 as usize;
        item.ObjType = OBJECTTYPE_STARSERVICE;
        item.ObjData_StarService = STARRC_WEAK!();
        item.SObjData_StarService = In_ObjData.clone();
        /*--if old exist, replace old--*/
        if RefCount == 0 { 
            l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
        }else{
            l_WeakTable.insert(RefCount, item);
        }          
        let NewRefCount : usize; 
        if RefCount == 0 {
            NewRefCount = G_WEAKTABLE_REFCOUNT;
            G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
        }else{
            NewRefCount = RefCount;                  
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return NewRefCount;
    }
}

/*---input is raw pointer---*/
/*  usage:
    let s_refval = NewRustObjectRefStrong_StarParaPkg(&Rc<StarParaPkg>, 0);            
*/

fn NewRustObjectRefStrong_StarParaPkg(In_ObjData: &STARPARAPKG, RefCount: usize) -> usize {
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarParaPkg.upgrade();
                if sss.is_some() {
                    //olditem.ObjData_StarParaPkg = Rc::downgrade(&sss.unwrap());
                    //panic!("strong reference type error, it holds weak ref")
                }else if let Some(ObjData) = STARGETRCREF_STARPARAPKG!(olditem.SObjData_StarParaPkg) {
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                } else {
                    panic!("strong reference type error")
                }
            }
        }
        let mut item = _StarCore_WeakTableItem::new();
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARPARAPKG!(In_ObjData); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            if RefCount == 0 {
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
            }else{
                ObjData.ObjData.RefItem = RefCount;
            }
        }
        item.RefCount = 0 as usize;
        item.ObjType = OBJECTTYPE_STARPARAPKG;
        item.ObjData_StarParaPkg = STARRC_WEAK!();
        item.SObjData_StarParaPkg = In_ObjData.clone();
        /*--if old exist, replace old--*/
        if RefCount == 0 { 
            l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
        }else{
            l_WeakTable.insert(RefCount, item);
        }          
        let NewRefCount : usize; 
        if RefCount == 0 {
            NewRefCount = G_WEAKTABLE_REFCOUNT;
            G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
        }else{
            NewRefCount = RefCount;                   
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return NewRefCount;
    }
}

/*---input is raw pointer---*/
/*  usage:
    let s_refval = NewRustObjectRefStrong_StarBinBuf(&Rc<StarBinBuf>, 0);            
*/

fn NewRustObjectRefStrong_StarBinBuf(In_ObjData: &STARBINBUF, RefCount: usize) -> usize {
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
               let sss = olditem.ObjData_StarBinBuf.upgrade();
                if sss.is_some() {
                    //olditem.ObjData_StarBinBuf = Rc::downgrade(&sss.unwrap());
                    //panic!("strong reference type error, it holds weak ref")
                }else if let Some(ObjData) = STARGETRCREF_STARBINBUF!(olditem.SObjData_StarBinBuf) {                
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                } else {
                    panic!("strong reference type error")
                }
            }
        }
        let mut item = _StarCore_WeakTableItem::new();
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARBINBUF!(In_ObjData); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            if RefCount == 0 {
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
            }else{
                ObjData.ObjData.RefItem = RefCount;
            }
        }
        item.RefCount = 0 as usize;
        item.ObjType = OBJECTTYPE_STARBINBUF;
        item.ObjData_StarBinBuf = STARRC_WEAK!();
        item.SObjData_StarBinBuf = In_ObjData.clone();
        /*--if old exist, replace old--*/
        if RefCount == 0 { 
            l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
        }else{
            l_WeakTable.insert(RefCount, item);
        }          
        let NewRefCount : usize; 
        if RefCount == 0 {
            NewRefCount = G_WEAKTABLE_REFCOUNT;
            G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
        }else{
            NewRefCount = RefCount;                  
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return NewRefCount;
    }
}

/*---input is raw pointer---*/
/*  usage:
    let s_refval = NewRustObjectRefStrong_StarSXml(&Rc<StarSXml>, 0);            
*/

fn NewRustObjectRefStrong_StarSXml(In_ObjData: &STARSXML, RefCount: usize) -> usize {
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarSXml.upgrade();
                if sss.is_some() {
                    //olditem.ObjData_StarSXml = Rc::downgrade(&sss.unwrap());            
                    //panic!("strong reference type error, it holds weak ref")
                }else if let Some(ObjData) = STARGETRCREF_STARSXML!(olditem.SObjData_StarSXml) {   
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                } else {
                    panic!("strong reference type error")
                }
            }
        }
        let mut item = _StarCore_WeakTableItem::new();
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSXML!(In_ObjData); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            if RefCount == 0 {
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
            }else{
                ObjData.ObjData.RefItem = RefCount;
            }
        }
        item.RefCount = 0 as usize;
        item.ObjType = OBJECTTYPE_STARSXML;
        item.ObjData_StarSXml = STARRC_WEAK!();
        item.SObjData_StarSXml = In_ObjData.clone();
        /*--if old exist, replace old--*/
        if RefCount == 0 { 
            l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
        }else{
            l_WeakTable.insert(RefCount, item);
        }          
        let NewRefCount : usize; 
        if RefCount == 0 {
            NewRefCount = G_WEAKTABLE_REFCOUNT;
            G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
        }else{
            NewRefCount = RefCount;               
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return NewRefCount;
    }
}

/*---input is raw pointer---*/
/*  usage:
    let s_refval = NewRustObjectRefStrong_StarObject(&Rc<StarObject>, 0);            
*/

fn NewRustObjectRefStrong_StarObject(In_ObjData: &STAROBJECT, RefCount: usize) -> usize {
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarObject.upgrade();
                if sss.is_some() {
                    //olditem.ObjData_StarObject = Rc::downgrade(&sss.unwrap());               
                    //panic!("strong reference type error, it holds weak ref")
                }else if let Some(ObjData) = STARGETRCREF_STAROBJECT!(olditem.SObjData_StarObject) { 
                    olditem.RefCount = olditem.RefCount + 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RefCount;
                } else {
                    panic!("strong reference type error")
                }
            }
        }
        let mut item = _StarCore_WeakTableItem::new();
        let mut STARGETRCMUT_temp = STARGETRCMUT_STAROBJECT!(In_ObjData); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            if RefCount == 0 {
                ObjData.ObjData.RefItem = G_WEAKTABLE_REFCOUNT;
            }else{
                ObjData.ObjData.RefItem = RefCount;
            }
        }
        item.RefCount = 0 as usize;
        item.ObjType = OBJECTTYPE_STAROBJECT;
        item.ObjData_StarObject = STARRC_WEAK!();
        item.SObjData_StarObject = In_ObjData.clone();
        /*--if old exist, replace old--*/
        if RefCount == 0 { 
            l_WeakTable.insert(G_WEAKTABLE_REFCOUNT, item);
        }else{
            l_WeakTable.insert(RefCount, item);
        }          
        let NewRefCount : usize; 
        if RefCount == 0 {
            NewRefCount = G_WEAKTABLE_REFCOUNT;
            G_WEAKTABLE_REFCOUNT = G_WEAKTABLE_REFCOUNT + 1;
        }else{
            NewRefCount = RefCount;               
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return NewRefCount;
    }
}

/*
usage:
    let cpt = SRefToRustObject_StarSrvGroup(refval);
*/
fn SRefToRustObject_StarSrvGroup(RefCount: usize) -> STARSRVGROUP {
    if RefCount == 0 {
        return STARRC!(None);
    }
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarSrvGroup.upgrade();
                if sss.is_some() {
                    let ObjData_StarSrvGroup = sss.unwrap();   
                    let RetVal = ObjData_StarSrvGroup.clone();
                    olditem.ObjData_StarSrvGroup = Rc::downgrade(&ObjData_StarSrvGroup); 
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RetVal;   
                } else {
                    /*--strong ref--*/
                    let ret = olditem.SObjData_StarSrvGroup.clone();
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return ret;
                }
            }
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return STARRC!(None);
    }
}

/*
usage:
    let a2 = SRefToRustObject_StarService(refval);
*/
fn SRefToRustObject_StarService(RefCount: usize) -> STARSERVICE {
    if RefCount == 0 {
        return STARRC!(None);
    }
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarService.upgrade();
                if sss.is_some() {
                    let ObjData_StarService = sss.unwrap();   
                    let RetVal = ObjData_StarService.clone();
                    olditem.ObjData_StarService = Rc::downgrade(&ObjData_StarService);
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RetVal;   
                } else {
                    /*--strong ref--*/
                    let ret = olditem.SObjData_StarService.clone();
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return ret;
                }
            }
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return STARRC!(None);
    }
}

/*
usage:
    let a2 = SRefToRustObject_StarParaPkg(refval);
*/
fn SRefToRustObject_StarParaPkg(RefCount: usize) -> STARPARAPKG {
    if RefCount == 0 {
        return STARRC!(None);
    }
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarParaPkg.upgrade();
                if sss.is_some() {
                    let ObjData_StarParaPkg = sss.unwrap();   
                    let RetVal = ObjData_StarParaPkg.clone();
                    olditem.ObjData_StarParaPkg = Rc::downgrade(&ObjData_StarParaPkg);
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RetVal;   
                } else {
                    /*--strong ref--*/
                    let ret = olditem.SObjData_StarParaPkg.clone();
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return ret;
                }
            }
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return STARRC!(None);
    }
}

/*
usage:
    let a2 = SRefToRustObject_StarBinBuf(refval);
*/
fn SRefToRustObject_StarBinBuf(RefCount: usize) -> STARBINBUF {
    if RefCount == 0 {
        return STARRC!(None);
    }
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarBinBuf.upgrade();
                if sss.is_some() {
                    let ObjData_StarBinBuf = sss.unwrap();   
                    let RetVal = ObjData_StarBinBuf.clone();
                    olditem.ObjData_StarBinBuf = Rc::downgrade(&ObjData_StarBinBuf);
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RetVal;   
                } else {
                    /*--strong ref--*/
                    let ret = olditem.SObjData_StarBinBuf.clone();
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return ret;
                }
            }
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return STARRC!(None);
    }
}

/*
usage:
    let a2 = SRefToRustObject_StarSXml(refval);
*/
fn SRefToRustObject_StarSXml(RefCount: usize) -> STARSXML {
    if RefCount == 0 {
        return STARRC!(None);
    }
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarSXml.upgrade();
                if sss.is_some() {
                    let ObjData_StarSXml = sss.unwrap();   
                    let RetVal = ObjData_StarSXml.clone();
                    olditem.ObjData_StarSXml = Rc::downgrade(&ObjData_StarSXml);
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RetVal;   
                } else {
                    /*--strong ref--*/
                    let ret = olditem.SObjData_StarSXml.clone();
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return ret;
                }
            }
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return STARRC!(None);
    }
}

/*
usage:
    let a2 = SRefToRustObject_StarObject(refval);
*/
fn SRefToRustObject_StarObject(RefCount: usize) -> STAROBJECT {
    if RefCount == 0 {
        return STARRC!(None);
    }
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                let sss = olditem.ObjData_StarObject.upgrade();
                if sss.is_some() {
                    let ObjData_StarObject = sss.unwrap();   
                    let RetVal = ObjData_StarObject.clone();
                    olditem.ObjData_StarObject = Rc::downgrade(&ObjData_StarObject);
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return RetVal;   
                } else {
                    /*--strong ref--*/
                    let ret = olditem.SObjData_StarObject.clone();
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return ret;
                }
            }
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return STARRC!(None);
    }
}

/*---delete reference--*/
fn DeleteRustObjectRef(RefCount: usize) {
    if RefCount == 0 {
        return;
    }
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();
                if olditem.RefCount > 0 {
                    olditem.RefCount = olditem.RefCount - 1;
                    starrust_mutex_unlock(WEAKTABLEMUTEX);
                    return;
                }
                
                if olditem.ObjType == OBJECTTYPE_STARSRVGROUP {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(olditem.SObjData_StarSrvGroup); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STARSERVICE {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARSERVICE!(olditem.SObjData_StarService); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STARPARAPKG {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARPARAPKG!(olditem.SObjData_StarParaPkg); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STARBINBUF {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARBINBUF!(olditem.SObjData_StarBinBuf); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STARSXML {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARSXML!(olditem.SObjData_StarSXml); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STAROBJECT {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STAROBJECT!(olditem.SObjData_StarObject); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }                                                                                
                } 

                olditem.ObjData_StarSrvGroup = STARRC_WEAK!();
                olditem.ObjData_StarService = STARRC_WEAK!();
                olditem.ObjData_StarParaPkg = STARRC_WEAK!();
                olditem.ObjData_StarBinBuf = STARRC_WEAK!();
                olditem.ObjData_StarSXml = STARRC_WEAK!();
                olditem.ObjData_StarObject = STARRC_WEAK!();

                olditem.SObjData_StarSrvGroup = STARRC!(None);
                olditem.SObjData_StarService = STARRC!(None);
                olditem.SObjData_StarParaPkg = STARRC!(None);
                olditem.SObjData_StarBinBuf = STARRC!(None);
                olditem.SObjData_StarSXml = STARRC!(None);
                olditem.SObjData_StarObject = STARRC!(None);

                /* l_WeakTable.remove(&RefCount); Cause compilation errors, Is it strange? */
                let l_WeakTable2 =
                    &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
                l_WeakTable2.remove(&RefCount);                

                starrust_mutex_unlock(WEAKTABLEMUTEX);
                return;
            }
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return;
    }
}

/*---delete reference--*/
fn DeleteRustObjectAllRef(RefCount: usize) {
    if RefCount == 0 {
        return;
    }
    unsafe {
        starrust_mutex_lock(WEAKTABLEMUTEX);
        if G_WEAKTABLE == 0 as *mut _ {
            G_WEAKTABLE = Box::into_raw(Box::new(HashMap::new()));
        }
        let l_WeakTable = &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
        if RefCount != 0 {
            let ok = l_WeakTable.contains_key(&RefCount);
            if ok {
                let olditem = l_WeakTable.get_mut(&RefCount).unwrap();

                if olditem.ObjType == OBJECTTYPE_STARSRVGROUP {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(olditem.SObjData_StarSrvGroup); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STARSERVICE {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARSERVICE!(olditem.SObjData_StarService); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STARPARAPKG {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARPARAPKG!(olditem.SObjData_StarParaPkg); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STARBINBUF {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARBINBUF!(olditem.SObjData_StarBinBuf); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STARSXML {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARSXML!(olditem.SObjData_StarSXml); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }
                }else if olditem.ObjType == OBJECTTYPE_STAROBJECT {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STAROBJECT!(olditem.SObjData_StarObject); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        ObjData.ObjData.RefItem = 0;
                    }                                                                                
                }                
                
                olditem.ObjData_StarSrvGroup = STARRC_WEAK!();
                olditem.ObjData_StarService = STARRC_WEAK!();
                olditem.ObjData_StarParaPkg = STARRC_WEAK!();
                olditem.ObjData_StarBinBuf = STARRC_WEAK!();
                olditem.ObjData_StarSXml = STARRC_WEAK!();
                olditem.ObjData_StarObject = STARRC_WEAK!();

                olditem.SObjData_StarSrvGroup = STARRC!(None);
                olditem.SObjData_StarService = STARRC!(None);
                olditem.SObjData_StarParaPkg = STARRC!(None);
                olditem.SObjData_StarBinBuf = STARRC!(None);
                olditem.SObjData_StarSXml = STARRC!(None);
                olditem.SObjData_StarObject = STARRC!(None);

                /* l_WeakTable.remove(&RefCount); Cause compilation errors, Is it strange? */
                let l_WeakTable2 =
                    &mut *(G_WEAKTABLE as *mut HashMap<usize, _StarCore_WeakTableItem>);
                l_WeakTable2.remove(&RefCount);                

                starrust_mutex_unlock(WEAKTABLEMUTEX);
                return;
            }
        }
        starrust_mutex_unlock(WEAKTABLEMUTEX);
        return;
    }
}

/*-----------------------------------------------------------------------*/
struct StarServiceBody {
    ServiceGroupID: u32,
    SRPInterface: *const c_void,
    IsClearedByStarCore: bool,
    RefItem: usize,
}
impl StarServiceBody {
    pub fn new() -> StarServiceBody {
        StarServiceBody {
            ServiceGroupID: 0 as u32,
            SRPInterface: 0 as *const c_void,
            IsClearedByStarCore: false,
            RefItem: 0 as usize,
        }
    }
}

impl Clone for StarServiceBody {
    fn clone(&self) -> Self {
        let mut aaa = StarServiceBody::new();
        aaa.ServiceGroupID = self.ServiceGroupID.clone();
        aaa.SRPInterface = self.SRPInterface;
        aaa.IsClearedByStarCore = self.IsClearedByStarCore;
        aaa.RefItem = self.RefItem;
        return aaa;
    }
}

pub struct StarService {
    IsValid: bool,
    ObjData: StarServiceBody,
}
impl StarService {
    pub fn new() -> StarService {
        StarService {
            IsValid: false,
            ObjData: StarServiceBody::new(),
        }
    }
}

fn ToStarService(fdata: &mut StarSrvGroupBody, SRPInterface: *mut c_void) -> STARSERVICE {
    let mut SRPInterfaceItem: StructOfSRPSrvGroup_SRPInterfaceItem;
    let mut ServiceID: VS_UUID = unsafe { std::mem::zeroed() };

    if SRPInterface == 0 as *mut c_void {
        return STARRC!(None);
    }
    unsafe {
        Star_SRPI_GetServiceID(SRPInterface, &mut ServiceID);
    }
    SRPInterfaceItem = StructOfSRPSrvGroup_SRPInterfaceItem::new();
    for _t in &mut fdata.SRPInterfaceItemRoot {
        if unsafe { starrust_uuidisequal(&_t.ServiceID, &ServiceID) } == VS_TRUE {
            SRPInterfaceItem = _t.clone();
            break;
        }
    }
    let mut NeedPush: bool = false;
    if SRPInterfaceItem != StructOfSRPSrvGroup_SRPInterfaceItem::new() {
        DeleteRustObjectAllRef(SRPInterfaceItem.ServiceObject);
        SRPInterfaceItem.ServiceObject = 0;
    } else {
        SRPInterfaceItem = StructOfSRPSrvGroup_SRPInterfaceItem::new();
        NeedPush = true;
    }

    let mut s = StarService::new();
    {
        /*little tricks*/
        let m_StarServiceBody = &mut s.ObjData;
        m_StarServiceBody.SRPInterface = SRPInterface;
        m_StarServiceBody.ServiceGroupID = fdata.ServiceGroupID;
        m_StarServiceBody.IsClearedByStarCore = false;
        m_StarServiceBody.RefItem = 0 as usize;
        s.IsValid = true;
    }
    let mut rs = STARRC!(Some(s));
    SRPInterfaceItem.ServiceObject = NewRustObjectRefStrong_StarService(&mut rs, 0);  /* change to strong ref--*/
    SRPInterfaceItem.ServiceID = ServiceID;
    if NeedPush == true {
        fdata.SRPInterfaceItemRoot.push(SRPInterfaceItem);
    }

    return rs;
}

impl fmt::Display for StarService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val: *mut c_char;
        let st: String;

        if self.ObjData.SRPInterface == 0 as *const c_void {
            st = "service object is invalid".to_owned();
        } else {
            val = unsafe { Star_SRPI_GetServiceName(self.ObjData.SRPInterface as *mut c_void) };
            if val == 0 as *mut c_char {
                st = "service not load".to_owned();
            } else {
                st = unsafe { CStr::from_ptr(val) }.to_str().unwrap().to_owned();
            }
        }
        write!(f, "{}", st)
    }
}

impl Drop for StarService {
    fn drop(&mut self) {
        DeleteRustObjectAllRef(self.ObjData.RefItem);
        if self.IsValid == true {
            _TermObject_Defer(&mut self.ObjData as &mut StarServiceBody);
        }
    }
}

/*-----------------------------------------------------------------------*/
struct StructOfSRPSrvGroup_SRPInterfaceItem {
    ServiceID: VS_UUID,
    ServiceObject: usize, /*ref*/
}
impl StructOfSRPSrvGroup_SRPInterfaceItem {
    pub fn new() -> StructOfSRPSrvGroup_SRPInterfaceItem {
        StructOfSRPSrvGroup_SRPInterfaceItem {
            ServiceID: VS_UUID::new(),
            ServiceObject: 0 as usize, /*ref*/
        }
    }
}

impl PartialEq for StructOfSRPSrvGroup_SRPInterfaceItem {
    fn eq(&self, other: &StructOfSRPSrvGroup_SRPInterfaceItem) -> bool {
        if unsafe { starrust_uuidisequal(&self.ServiceID, &other.ServiceID) == VS_TRUE }
            && self.ServiceObject == other.ServiceObject
        {
            return true;
        }
        return false;
    }
}

impl Clone for StructOfSRPSrvGroup_SRPInterfaceItem {
    fn clone(&self) -> Self {
        let mut aaa = StructOfSRPSrvGroup_SRPInterfaceItem::new();
        aaa.ServiceID = self.ServiceID.clone();
        aaa.ServiceObject = self.ServiceObject;
        return aaa;
    }
}

struct StarSrvGroupBody {
    ServiceGroupID: u32,
    BasicSRPInterface: *const c_void,
    SRPInterfaceItemRoot: Vec<StructOfSRPSrvGroup_SRPInterfaceItem>,
    ObjectIndexTree: *const c_void, /* save StructOfObjectRefInSrvGroup */
    RefItem: usize,
}
impl StarSrvGroupBody {
    pub fn new() -> StarSrvGroupBody {
        StarSrvGroupBody {
            ServiceGroupID: 0 as u32,
            BasicSRPInterface: 0 as *const c_void,
            SRPInterfaceItemRoot: Vec::new(),
            ObjectIndexTree: 0 as *const c_void, /* save StructOfObjectRefInSrvGroup */
            RefItem: 0 as usize,
        }
    }
    fn clone(&self) -> StarSrvGroupBody {  /*--only simple field--*/
        let mut aaa = StarSrvGroupBody::new();
        aaa.ServiceGroupID = self.ServiceGroupID;
        aaa.BasicSRPInterface = self.BasicSRPInterface;
        aaa.ObjectIndexTree = self.ObjectIndexTree;
        aaa.RefItem = self.RefItem;
        return aaa;
    }
}

pub struct StarSrvGroup {
    IsValid: bool,
    ObjData: StarSrvGroupBody,
}
impl StarSrvGroup {
    pub fn new() -> StarSrvGroup {
        StarSrvGroup {
            IsValid: false,
            ObjData: StarSrvGroupBody::new(),
        }
    }
}

fn ToStarSrvGroup(ServiceGroupID: u32, BasicSRPInterface: *mut c_void) -> STARSRVGROUP {
    let mut s = StarSrvGroup::new();
    {
        let m_StarSrvGroupBody = &mut s.ObjData;
        m_StarSrvGroupBody.BasicSRPInterface = BasicSRPInterface;
        m_StarSrvGroupBody.ServiceGroupID = ServiceGroupID;
        m_StarSrvGroupBody.ObjectIndexTree =
            unsafe { Star_SRPBasic_CreateIDIndex_Nor(BasicSRPInterface, 0) };
    }
    let mut rs = STARRC!(Some(s));
    NewRustObjectRef_StarSrvGroup(&mut rs, 0);
    let RefItem = STARSRVGROUP_RefItem(&rs);

    unsafe {
        Star_SRPBasic_RegObjectIDChangeNotify(
            BasicSRPInterface,
            starrust_ObjectIDChangeNotify as *const c_void,
            RefItem,
        );
    }
    unsafe {
        Star_SRPBasic_RegObjectFreeNotify(
            BasicSRPInterface,
            starrust_ObjectFreeNotify as *const c_void,
            RefItem,
        );
    }

    return rs;
}

impl fmt::Display for StarSrvGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val: *mut c_char;
        let st: String;

        if self.ObjData.BasicSRPInterface == 0 as *const c_void {
            st = "srvgroup object is invalid".to_owned();
        } else {
            val = unsafe {
                Star_SRPBasic_QueryActiveService(
                    self.ObjData.BasicSRPInterface as *mut c_void,
                    0 as *mut VS_UUID,
                )
            };
            if val == 0 as *mut c_char {
                st = "service not load".to_owned();
            } else {
                st = unsafe { CStr::from_ptr(val) }.to_str().unwrap().to_owned();
            }
        }
        write!(f, "{}", st)
    }
}

impl Drop for StarSrvGroup {
    fn drop(&mut self) {
        DeleteRustObjectAllRef(self.ObjData.RefItem);
        if self.IsValid == true {
            _TermObject_Defer(&mut self.ObjData as &mut StarSrvGroupBody);
        }
    }
}

/*-----------------------------------------------------------------------*/
struct StarParaPkgBody {
    ParaPkg: *const c_void,
    ServiceGroupID: u32,
    FreeFlag: bool,
    IsClearedByStarCore: bool,
    RefItem: usize,
}

impl StarParaPkgBody {
    pub fn new() -> StarParaPkgBody {
        StarParaPkgBody {
            ParaPkg: 0 as *const c_void,
            ServiceGroupID: 0u32,
            FreeFlag: false,
            IsClearedByStarCore: false,
            RefItem: 0usize,
        }
    }
}
impl Clone for StarParaPkgBody {
    fn clone(&self) -> Self {
        let mut aaa = StarParaPkgBody::new();
        aaa.ParaPkg = self.ParaPkg;
        aaa.ServiceGroupID = self.ServiceGroupID;
        aaa.FreeFlag = self.FreeFlag;
        aaa.IsClearedByStarCore = self.IsClearedByStarCore;
        aaa.RefItem = self.RefItem;
        return aaa;
    }
}

pub struct StarParaPkg {
    IsValid: bool,
    ObjData: StarParaPkgBody,
}
impl StarParaPkg {
    pub fn new() -> StarParaPkg {
        StarParaPkg {
            IsValid: false,
            ObjData: StarParaPkgBody::new(),
        }
    }
}

fn ToStarParaPkg(ParaPkg: *mut c_void, ServiceGroupID: u32, FreeFlag: bool) -> STARPARAPKG {
    let mut s = StarParaPkg::new();
    {
        let m_StarParaPkgBody = &mut s.ObjData;
        m_StarParaPkgBody.ParaPkg = ParaPkg;
        m_StarParaPkgBody.ServiceGroupID = ServiceGroupID;
        m_StarParaPkgBody.FreeFlag = FreeFlag;
        m_StarParaPkgBody.IsClearedByStarCore = false;
        if m_StarParaPkgBody.FreeFlag == false {
            unsafe {
                Star_SRPParaPkg_AddRef(m_StarParaPkgBody.ParaPkg as *mut c_void);
            }
            m_StarParaPkgBody.FreeFlag = true;
        }
    }
    let mut rs = STARRC!(Some(s));
    NewRustObjectRef_StarParaPkg(&mut rs, 0);
    let RefItem = STARPARAPKG_RefItem(&rs);

    unsafe {
        Star_SRPControl_RegScriptObject(
            StarRust_SRPControlInterface,
            RefItem,
            starrust_FreeScriptObject as *mut c_void,
            0,
        );
    }
    return rs;
}

fn CreateStarParaPkg(BasicSRPInterface: *mut c_void) -> STARPARAPKG {
    let ParaPkg = unsafe { Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface) };
    if ParaPkg == 0 as *mut c_void {
        return STARRC!(None);
    }
    return ToStarParaPkg(
        ParaPkg,
        unsafe { Star_SRPBasic_GetServiceGroupID(BasicSRPInterface) },
        true,
    );
}

impl fmt::Display for StarParaPkg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parapkg")
    }
}

impl Drop for StarParaPkg {
    fn drop(&mut self) {
        DeleteRustObjectAllRef(self.ObjData.RefItem);
        if self.IsValid == true {
            _TermObject_Defer(&mut self.ObjData as &mut StarParaPkgBody);
        }
    }
}

/*-----------------------------------------------------------------------*/
struct StarBinBufBody {
    BinBuf: *const c_void,
    ServiceGroupID: u32,
    FreeFlag: bool,
    IsClearedByStarCore: bool,
    RefItem: usize,
}

impl StarBinBufBody {
    pub fn new() -> StarBinBufBody {
        StarBinBufBody {
            BinBuf: 0 as *const c_void,
            ServiceGroupID: 0u32,
            FreeFlag: false,
            IsClearedByStarCore: false,
            RefItem: 0usize,
        }
    }
}

impl Clone for StarBinBufBody {
    fn clone(&self) -> Self {
        let mut aaa = StarBinBufBody::new();
        aaa.BinBuf = self.BinBuf;
        aaa.ServiceGroupID = self.ServiceGroupID;
        aaa.FreeFlag = self.FreeFlag;
        aaa.IsClearedByStarCore = self.IsClearedByStarCore;
        aaa.RefItem = self.RefItem;
        return aaa;
    }
}

pub struct StarBinBuf {
    IsValid: bool,
    ObjData: StarBinBufBody,
}
impl StarBinBuf {
    pub fn new() -> StarBinBuf {
        StarBinBuf {
            IsValid: false,
            ObjData: StarBinBufBody::new(),
        }
    }
}

fn ToStarBinBuf(BinBuf: *mut c_void, ServiceGroupID: u32, FreeFlag: bool) -> STARBINBUF {
    let mut s = StarBinBuf::new();
    {
        let m_StarBinBufBody = &mut s.ObjData;
        m_StarBinBufBody.BinBuf = BinBuf;
        m_StarBinBufBody.ServiceGroupID = ServiceGroupID;
        m_StarBinBufBody.FreeFlag = FreeFlag;
        m_StarBinBufBody.IsClearedByStarCore = false;
        if m_StarBinBufBody.FreeFlag == false {
            unsafe {
                Star_SRPParaPkg_AddRef(m_StarBinBufBody.BinBuf as *mut c_void);
            }
            m_StarBinBufBody.FreeFlag = true;
        }
    }
    let mut rs = STARRC!(Some(s));
    NewRustObjectRef_StarBinBuf(&mut rs, 0);
    let RefItem = STARBINBUF_RefItem(&rs);

    unsafe {
        Star_SRPControl_RegScriptObject(
            StarRust_SRPControlInterface,
            RefItem,
            starrust_FreeScriptObject as *mut c_void,
            0,
        );
    }
    return rs;
}

fn CreateStarBinBuf(BasicSRPInterface: *mut c_void) -> STARBINBUF {
    let BinBuf = unsafe { Star_SRPBasic_GetSRPBinBufInterface(BasicSRPInterface) };
    if BinBuf == 0 as *mut c_void {
        return STARRC!(None);
    }
    return ToStarBinBuf(
        BinBuf,
        unsafe { Star_SRPBasic_GetServiceGroupID(BasicSRPInterface) },
        true,
    );
}

impl fmt::Display for StarBinBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*	missing GetName function for v2.5.2
        	*/

        write!(f, "binbuf")
    }
}

impl Drop for StarBinBuf {
    fn drop(&mut self) {
        DeleteRustObjectAllRef(self.ObjData.RefItem);
        if self.IsValid == true {
            _TermObject_Defer(&mut self.ObjData as &mut StarBinBufBody);
        }
    }
}

/*-----------------------------------------------------------------------*/
struct StarSXmlBody {
    SXml: *const c_void,
    ServiceGroupID: u32,
    FreeFlag: bool,
    IsClearedByStarCore: bool,
    RefItem: usize,
}

impl StarSXmlBody {
    pub fn new() -> StarSXmlBody {
        StarSXmlBody {
            SXml: 0 as *const c_void,
            ServiceGroupID: 0u32,
            FreeFlag: false,
            IsClearedByStarCore: false,
            RefItem: 0usize,
        }
    }
}
impl Clone for StarSXmlBody {
    fn clone(&self) -> Self {
        let mut aaa = StarSXmlBody::new();
        aaa.SXml = self.SXml;
        aaa.ServiceGroupID = self.ServiceGroupID;
        aaa.FreeFlag = self.FreeFlag;
        aaa.IsClearedByStarCore = self.IsClearedByStarCore;
        aaa.RefItem = self.RefItem;
        return aaa;
    }
}

pub struct StarSXml {
    IsValid: bool,
    ObjData: StarSXmlBody,
}
impl StarSXml {
    pub fn new() -> StarSXml {
        StarSXml {
            IsValid: false,
            ObjData: StarSXmlBody::new(),
        }
    }
}

fn ToStarSXml(SXml: *mut c_void, ServiceGroupID: u32, FreeFlag: bool) -> STARSXML {
    let mut s = StarSXml::new();
    {
        let StarSXmlBody = &mut s.ObjData;
        StarSXmlBody.SXml = SXml;
        StarSXmlBody.ServiceGroupID = ServiceGroupID;
        StarSXmlBody.FreeFlag = FreeFlag;
        StarSXmlBody.IsClearedByStarCore = false;
        if StarSXmlBody.FreeFlag == false {
            unsafe {
                Star_SRPParaPkg_AddRef(StarSXmlBody.SXml as *mut c_void);
            }
            StarSXmlBody.FreeFlag = true;
        }
    }
    let mut rs = STARRC!(Some(s));
    NewRustObjectRef_StarSXml(&mut rs, 0);
    let RefItem = STARSXML_RefItem(&rs);

    unsafe {
        Star_SRPControl_RegScriptObject(
            StarRust_SRPControlInterface,
            RefItem,
            starrust_FreeScriptObject as *mut c_void,
            0,
        );
    }
    return rs;
}

fn CreateStarSXml(BasicSRPInterface: *mut c_void) -> STARSXML {
    let SXml = unsafe { Star_SRPBasic_GetSXMLInterface(BasicSRPInterface) };
    if SXml == 0 as *mut c_void {
        return STARRC!(None);
    }
    return ToStarSXml(
        SXml,
        unsafe { Star_SRPBasic_GetServiceGroupID(BasicSRPInterface) },
        true,
    );
}

impl fmt::Display for StarSXml {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sxml")
    }
}

impl Drop for StarSXml {
    fn drop(&mut self) {
        DeleteRustObjectAllRef(self.ObjData.RefItem);
        if self.IsValid == true {
            _TermObject_Defer(&mut self.ObjData as &mut StarSXmlBody);
        }
    }
}

/*-----------------------------------------------------------------------*/
struct StarObjectBody {
    ObjectID: VS_UUID,
    ServiceGroupID: u32,
    NeedFreeFlag: bool,
    IsClearedByStarCore: bool,
    RefItem: usize,
    FuncTable: HashMap<String, fn(CleGroup:&STARSRVGROUP,CleService:&STARSERVICE,CleObject:&STAROBJECT,Paras: &[STARRESULT]) -> STARRESULT>, 
}

impl StarObjectBody {
    pub fn new() -> StarObjectBody {
        StarObjectBody {
            ObjectID: VS_UUID::new(),
            ServiceGroupID: 0u32,
            NeedFreeFlag: false,
            IsClearedByStarCore: false,
            RefItem: 0usize,
            FuncTable: HashMap::new(),
        }
    }
}

impl Clone for StarObjectBody {
    fn clone(&self) -> Self {
        let mut aaa = StarObjectBody::new();
        aaa.ObjectID = self.ObjectID.clone();
        aaa.ServiceGroupID = self.ServiceGroupID;
        aaa.NeedFreeFlag = self.NeedFreeFlag;
        aaa.IsClearedByStarCore = self.IsClearedByStarCore;
        aaa.RefItem = self.RefItem;
        //FuncTable: HashMap::new(), not clone
        return aaa;
    }
}

pub struct StarObject {
    IsValid: bool,
    ObjData: StarObjectBody,
}
impl StarObject {
    pub fn new() -> StarObject {
        StarObject {
            IsValid: false,
            ObjData: StarObjectBody::new(),
        }
    }
}

fn ToStarObject(
    SRPObject: *mut c_void,
    SRPInterface: *mut c_void,
    NeedFreeFlag: bool,
) -> STAROBJECT {
    let mut ObjectID: VS_UUID = unsafe { std::mem::zeroed() };

    if SRPObject == 0 as *mut c_void {
        return STARRC!(None);
    }
    unsafe {
        Star_SRPI_GetID(SRPInterface, SRPObject, &mut ObjectID);
    }
    let RetObject = RustSRPGetObject(
        unsafe { Star_SRPI_GetServiceGroupID(SRPInterface) },
        &mut ObjectID,
    );
    if STAROBJECT_IsValid(&RetObject) == true {
        if NeedFreeFlag == true {
            let s = format!(
                "object {} to rust script set as auto release again",
                unsafe { CStr::from_ptr(Star_SRPI_GetName(SRPInterface, SRPObject)) }
                    .to_str()
                    .unwrap()
                    .to_owned()
            );
            let c_s = CString::new(s).unwrap();
            unsafe {
                starrust_SRPI_ProcessError(
                    SRPInterface,
                    VSFAULT_WARNING,
                    CString::new("rust").unwrap().as_ptr(),
                    0,
                    c_s.as_ptr(),
                );
            }
        }
        return RetObject;
    }
    return _ToStarObject(
        unsafe { Star_SRPI_GetServiceGroupID(SRPInterface) },
        &mut ObjectID,
        NeedFreeFlag,
    );
}

fn ToStarObject_Basic(SRPObject: *const c_void,SRPInterface: *const c_void, NeedFreeFlag: bool) -> STAROBJECT
{
	if SRPObject == 0 as *const c_void {
		return STARRC!(None);
	}
    let mut ObjectID : VS_UUID = VS_UUID::new();
	unsafe{Star_SRPI_GetID(SRPInterface, SRPObject as *mut c_void, &mut ObjectID)};
	let RetObject = RustSRPGetObject(unsafe{Star_SRPI_GetServiceGroupID(SRPInterface)}, &mut ObjectID);
	if IsStarObjectClassObject(&RetObject) {
		if NeedFreeFlag == true {
            let s = format!(
                "object {} to rust script set as auto release again",
                unsafe { CStr::from_ptr(Star_SRPI_GetName(SRPInterface, SRPObject as *mut c_void)) }
                    .to_str()
                    .unwrap()
                    .to_owned()
            );
            let c_s = CString::new(s).unwrap();
            unsafe {
                starrust_SRPI_ProcessError(
                    SRPInterface,
                    VSFAULT_WARNING,
                    CString::new("rust").unwrap().as_ptr(),
                    0,
                    c_s.as_ptr(),
                );
            }
		}
		return RetObject;
	}
	return _ToStarObject(unsafe{Star_SRPI_GetServiceGroupID(SRPInterface)}, &mut ObjectID, NeedFreeFlag);
}

fn _ToStarObject(ServiceGroupID: u32, ObjectID: *mut VS_UUID, NeedFreeFlag: bool) -> STAROBJECT {
    let mut s = StarObject::new();
    {
        let m_StarObjectBody = &mut s.ObjData;
        m_StarObjectBody.ServiceGroupID = ServiceGroupID;
        m_StarObjectBody.ObjectID = unsafe { (*ObjectID).clone() };
        m_StarObjectBody.NeedFreeFlag = NeedFreeFlag;
        m_StarObjectBody.IsClearedByStarCore = false;
        m_StarObjectBody.FuncTable = HashMap::new();
        let SRPInterface = GetSRPServiceInterfaceEx(ServiceGroupID, ObjectID);
        if SRPInterface != 0 as *mut c_void {
            let SRPObject =
                unsafe { Star_SRPI_GetObject(SRPInterface, &mut m_StarObjectBody.ObjectID) };
            if SRPObject != 0 as *mut c_void {
                if m_StarObjectBody.NeedFreeFlag == false {
                    unsafe {
                        Star_SRPI_AddRefEx(SRPInterface, SRPObject);
                    }
                    m_StarObjectBody.NeedFreeFlag = true;
                }
            }
        }
    }
    let rs = STARRC!(Some(s));
    let RefItem = NewRustObjectRef_StarObject(&rs, 0);

    let mut sss_ObjData : StarObjectBody = StarObjectBody::new();

    if let Some(sss) = STARGETRCREF_STAROBJECT!(&rs).as_ref() {
        sss_ObjData = sss.ObjData.clone();
    }
    RustSRPSetObject(
        sss_ObjData.ServiceGroupID,
        &sss_ObjData.ObjectID,
        &rs,
        false,
    );
    unsafe {
        Star_SRPControl_RegScriptObject(
            StarRust_SRPControlInterface,
            RefItem,
            starrust_FreeScriptObject as *mut c_void,
            0,
        );
    }
    return rs;
}

impl fmt::Display for StarObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let st: String;
        let SRPInterface: *mut c_void;
        let SRPObject: *mut c_void;

        SRPInterface =
            GetSRPServiceInterfaceEx(self.ObjData.ServiceGroupID, &self.ObjData.ObjectID);
        if SRPInterface == 0 as *mut c_void {
            st = "unknown".to_owned();
        } else {
            let mut s_ObjectID = self.ObjData.ObjectID.clone();
            SRPObject = unsafe { Star_SRPI_GetObject(SRPInterface, &mut s_ObjectID) };
            if SRPObject == 0 as *mut c_void {
                st = "null".to_owned();
            } else {
                let CharPtr: *mut c_char;
                let mut InterfaceName: [*mut c_char; 1] = [0 as *mut c_char];

                CharPtr = unsafe {
                    Star_SRPI_GetRawContextType(
                        SRPInterface,
                        SRPObject,
                        &mut InterfaceName as *mut [*mut c_char; 1],
                    )
                };
                if CharPtr == 0 as *mut c_char {
                    let StrName = unsafe {
                        starrust_ToPointer(starrust_SRPI_ScriptCall(
                            SRPInterface,
                            SRPObject,
                            0 as *mut u32,
                            CString::new("_StarToString".to_owned()).unwrap().as_ptr(),
                            CString::new("()s".to_owned()).unwrap().as_ptr(), /*"()s"*/
                        ))
                    } as *mut c_char;
                    if StrName != 0 as *mut c_char {
                        st = SRPRustSetStr(StrName, false);
                    } else {
                        st = SRPRustSetStr(
                            unsafe { Star_SRPI_GetName(SRPInterface, SRPObject) },
                            false,
                        );
                    }
                } else {
                    let StrName = unsafe {
                        starrust_ToPointer(starrust_SRPI_ScriptCall(
                            SRPInterface,
                            SRPObject,
                            0 as *mut u32,
                            CString::new("_StarToString".to_owned()).unwrap().as_ptr(),
                            CString::new("()s".to_owned()).unwrap().as_ptr(), /*"()s"*/
                        ))
                    } as *mut c_char;
                    if StrName != 0 as *mut c_char {
                        st = format!(
                            "{}[{}:{}]{}",
                            unsafe { CStr::from_ptr(Star_SRPI_GetName(SRPInterface, SRPObject)) }
                                .to_str()
                                .unwrap()
                                .to_owned(),
                            unsafe { CStr::from_ptr(InterfaceName[0]) }
                                .to_str()
                                .unwrap()
                                .to_owned(),
                            unsafe { CStr::from_ptr(CharPtr) }
                                .to_str()
                                .unwrap()
                                .to_owned(),
                            unsafe { CStr::from_ptr(StrName) }
                                .to_str()
                                .unwrap()
                                .to_owned()
                        );
                    } else {
                        st = format!(
                            "{}[{}:{}]",
                            unsafe { CStr::from_ptr(Star_SRPI_GetName(SRPInterface, SRPObject)) }
                                .to_str()
                                .unwrap()
                                .to_owned(),
                            unsafe { CStr::from_ptr(InterfaceName[0]) }
                                .to_str()
                                .unwrap()
                                .to_owned(),
                            unsafe { CStr::from_ptr(CharPtr) }
                                .to_str()
                                .unwrap()
                                .to_owned()
                        );
                    }
                }
            }
        }
        write!(f, "{}", st)
    }
}

impl Drop for StarObject {
    fn drop(&mut self) {
        DeleteRustObjectAllRef(self.ObjData.RefItem);
        if self.IsValid == true {
            _TermObject_Defer(&mut self.ObjData as &mut StarObjectBody);
        }
    }
}

/*-----------------------------------------------------------------------*/
#[repr(C)]
struct StructOfObjectRefInSrvGroup {
    RefItem: usize,
    IncreaseRef: bool,
}
impl StructOfObjectRefInSrvGroup {
    pub fn new() -> StructOfObjectRefInSrvGroup {
        StructOfObjectRefInSrvGroup {
            RefItem: 0usize,
            IncreaseRef: false,
        }
    }
}

fn _TermObject_Defer(ObjData: &mut Any) {
    match ObjData.downcast_mut::<StarServiceBody>() {
        Some(fdata) => {
            if fdata.IsClearedByStarCore == false {
                if unsafe { StarRust_ModuleInitFlag == VS_TRUE }
                    && fdata.SRPInterface != 0 as *mut c_void
                {
                    unsafe {
                        Star_SRPI_Release(fdata.SRPInterface);
                    }
                    fdata.SRPInterface = 0 as *const c_void;
                }
                fdata.IsClearedByStarCore = true;
            }
            if unsafe {
                StarRust_ModuleInitFlag == VS_TRUE
                    && StarRust_SRPControlInterface != 0 as *mut c_void
            } && fdata.RefItem != 0
            {
                unsafe {
                    Star_SRPControl_UnRegScriptObject(
                        StarRust_SRPControlInterface,
                        fdata.RefItem,
                        starrust_FreeScriptObject as *mut c_void,
                        0,
                    );
                }
            }
            if fdata.RefItem != 0 {
                DeleteRustObjectAllRef(fdata.RefItem);
                fdata.RefItem = 0;
            }
        }
        None => {}
    }
    match ObjData.downcast_mut::<StarSrvGroupBody>() {
        Some(fdata) => {
            if fdata.BasicSRPInterface != 0 as *mut c_void && unsafe {
                StarRust_ModuleInitFlag == VS_TRUE
            } {
                if fdata.ObjectIndexTree != 0 as *mut c_void {
                    let mut QueryRecord: VS_QUERYRECORD = VS_QUERYRECORD::new();
                    let mut ObjectRefInSrvGroup = unsafe {
                        Star_SRPBasic_QueryFirstIDKey(
                            fdata.BasicSRPInterface,
                            fdata.ObjectIndexTree,
                            &mut QueryRecord,
                            0 as *mut VS_UUID,
                        )
                    }
                        as *mut StructOfObjectRefInSrvGroup;
                    loop {
                        if ObjectRefInSrvGroup == 0 as *mut StructOfObjectRefInSrvGroup {
                            break;
                        }
                        unsafe {
                            DeleteRustObjectAllRef((*ObjectRefInSrvGroup).RefItem);
                        }
                        unsafe {
                            starrust_FreeStructOfObjectRefInSrvGroup(ObjectRefInSrvGroup);
                        }
                        ObjectRefInSrvGroup = unsafe {
                            Star_SRPBasic_QueryNextIDKey(
                                fdata.BasicSRPInterface,
                                fdata.ObjectIndexTree,
                                &mut QueryRecord,
                                0 as *mut VS_UUID,
                            )
                        }
                            as *mut StructOfObjectRefInSrvGroup;
                    }
                    unsafe {
                        Star_SRPBasic_DestoryIndex(fdata.BasicSRPInterface, fdata.ObjectIndexTree);
                    }
                    fdata.ObjectIndexTree = 0 as *mut c_void;
                }
                unsafe {
                    Star_SRPBasic_UnRegObjectIDChangeNotify(
                        fdata.BasicSRPInterface,
                        starrust_ObjectIDChangeNotify as *const c_void,
                        fdata.RefItem,
                    );
                }
                unsafe {
                    Star_SRPBasic_UnRegObjectFreeNotify(
                        fdata.BasicSRPInterface,
                        starrust_ObjectFreeNotify as *const c_void,
                        fdata.RefItem,
                    );
                }

                //--SRPInterfaceItem
                for SRPInterfaceItem in fdata.SRPInterfaceItemRoot.iter_mut() {
                    DeleteRustObjectAllRef(SRPInterfaceItem.ServiceObject);
                }
                fdata.SRPInterfaceItemRoot.clear();
                unsafe {
                    Star_SRPBasic_Release(fdata.BasicSRPInterface);
                }
                fdata.BasicSRPInterface = 0 as *const c_void;
            }

            if fdata.ServiceGroupID != VS_INVALID_SERVICEGROUPID {
                RustSRPClearSrvGroupWithID(fdata.ServiceGroupID);
                fdata.ServiceGroupID = VS_INVALID_SERVICEGROUPID;
            }
            if fdata.RefItem != 0 {
                DeleteRustObjectAllRef(fdata.RefItem);
                fdata.RefItem = 0;
            }
            return;
        }
        None => {}
    }
    match ObjData.downcast_mut::<StarParaPkgBody>() {
        Some(fdata) => {
            if fdata.IsClearedByStarCore == false {
                if fdata.FreeFlag == true && fdata.ParaPkg != 0 as *mut c_void {
                    if unsafe { StarRust_ModuleInitFlag == VS_TRUE } {
                        unsafe {
                            Star_SRPParaPkg_Release(fdata.ParaPkg);
                        }
                    }
                    fdata.ParaPkg = 0 as *const c_void;
                }
                fdata.IsClearedByStarCore = true;
            }
            if unsafe {
                StarRust_ModuleInitFlag == VS_TRUE
                    && StarRust_SRPControlInterface != 0 as *mut c_void
            } && fdata.RefItem != 0
            {
                unsafe {
                    Star_SRPControl_UnRegScriptObject(
                        StarRust_SRPControlInterface,
                        fdata.RefItem,
                        starrust_FreeScriptObject as *mut c_void,
                        0,
                    );
                }
            }
            if fdata.RefItem != 0 {
                DeleteRustObjectAllRef(fdata.RefItem);
                fdata.RefItem = 0;
            }
        }
        None => {}
    }
    match ObjData.downcast_mut::<StarBinBufBody>() {
        Some(fdata) => {
            if fdata.IsClearedByStarCore == false {
                if fdata.FreeFlag == true && fdata.BinBuf != 0 as *mut c_void {
                    if unsafe { StarRust_ModuleInitFlag == VS_TRUE } {
                        unsafe {
                            Star_SRPBinBuf_Release(fdata.BinBuf);
                        }
                    }
                    fdata.BinBuf = 0 as *const c_void;
                }
                fdata.IsClearedByStarCore = true;
            }
            if unsafe {
                StarRust_ModuleInitFlag == VS_TRUE
                    && StarRust_SRPControlInterface != 0 as *mut c_void
            } && fdata.RefItem != 0
            {
                unsafe {
                    Star_SRPControl_UnRegScriptObject(
                        StarRust_SRPControlInterface,
                        fdata.RefItem,
                        starrust_FreeScriptObject as *mut c_void,
                        0,
                    );
                }
            }
            if fdata.RefItem != 0 {
                DeleteRustObjectAllRef(fdata.RefItem);
                fdata.RefItem = 0;
            }
        }
        None => {}
    }
    match ObjData.downcast_mut::<StarSXmlBody>() {
        Some(fdata) => {
            if fdata.IsClearedByStarCore == false {
                if fdata.FreeFlag == true && fdata.SXml != 0 as *mut c_void {
                    if unsafe { StarRust_ModuleInitFlag == VS_TRUE } {
                        unsafe {
                            Star_SRPSXML_Release(fdata.SXml);
                        }
                    }
                    fdata.SXml = 0 as *const c_void;
                }
                fdata.IsClearedByStarCore = true;
            }
            if unsafe {
                StarRust_ModuleInitFlag == VS_TRUE
                    && StarRust_SRPControlInterface != 0 as *mut c_void
            } && fdata.RefItem != 0
            {
                unsafe {
                    Star_SRPControl_UnRegScriptObject(
                        StarRust_SRPControlInterface,
                        fdata.RefItem,
                        starrust_FreeScriptObject as *mut c_void,
                        0,
                    );
                }
            }
            if fdata.RefItem != 0 {
                DeleteRustObjectAllRef(fdata.RefItem);
                fdata.RefItem = 0;
            }
        }
        None => {}
    }
    match ObjData.downcast_mut::<StarObjectBody>() {
        Some(fdata) => {
            let SRPInterface: *const c_void;
            let SRPObject: *const c_void;
            if fdata.IsClearedByStarCore == false {
                if unsafe { StarRust_ModuleInitFlag == VS_TRUE } {
                    RustSRPClearObject(fdata.ServiceGroupID, &mut fdata.ObjectID);

                    SRPInterface = GetSRPServiceInterfaceEx(fdata.ServiceGroupID, &fdata.ObjectID);
                    if SRPInterface != 0 as *const c_void {
                        SRPObject =
                            unsafe { Star_SRPI_GetObject(SRPInterface, &mut fdata.ObjectID) };
                        if SRPObject != 0 as *const c_void {
                            unsafe {
                                Star_SRPI_UnRegLuaFuncEx(
                                    SRPInterface,
                                    SRPObject as *mut c_void,
                                    starrust_SRPObject_ScriptCallBack as *mut c_void,
                                    fdata.RefItem,
                                );
                            }
                            if fdata.NeedFreeFlag == true {
                                unsafe {
                                    Star_SRPI_UnLockGC(SRPInterface, SRPObject as *mut c_void)
                                };
                            }
                        }
                    }
                }
                fdata.IsClearedByStarCore = true;
            }
            if unsafe {
                StarRust_ModuleInitFlag == VS_TRUE
                    && StarRust_SRPControlInterface != 0 as *mut c_void
            } && fdata.RefItem != 0
            {
                unsafe {
                    Star_SRPControl_UnRegScriptObject(
                        StarRust_SRPControlInterface,
                        fdata.RefItem,
                        starrust_FreeScriptObject as *mut c_void,
                        0,
                    );
                }
            }
            if fdata.RefItem != 0 {
                DeleteRustObjectAllRef(fdata.RefItem);
                fdata.RefItem = 0;
            }
        }
        None => {}
    }
}

fn TOVS_BOOL(val: bool) -> VS_BOOL {
    if val {
        return VS_TRUE;
    }
    return VS_FALSE;
}

fn FROMVS_BOOL(val: VS_BOOL) -> bool {
    if val == VS_FALSE {
        return false;
    }
    return true;
}

fn SRPRustGetInt(Obj: &Any, ProcessFloat: bool) -> (u32, bool) {
    if let Some(fdata) = Obj.downcast_ref::<bool>() {
        if *fdata {
            return (1, true);
        } else {
            return (0, true);
        }
    } else if let Some(fdata) = Obj.downcast_ref::<i8>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<u8>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<i16>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<u16>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<i32>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<u32>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<isize>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<usize>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<i64>() {
        return (*fdata as u32, true);
    } else if let Some(fdata) = Obj.downcast_ref::<u64>() {
        return (*fdata as u32, true);
    }
    if ProcessFloat == true {
        if let Some(fdata) = Obj.downcast_ref::<f32>() {
            return (*fdata as u32, true);
        } else if let Some(fdata) = Obj.downcast_ref::<f64>() {
            return (*fdata as u32, true);
        }
    }
    return (0, false);
}

fn SRPRustToInt(Obj: &Any, ProcessFloat: bool) -> u32 {
    if let Some(fdata) = Obj.downcast_ref::<bool>() {
        if *fdata {
            return 1;
        } else {
            return 0;
        }
    } else if let Some(fdata) = Obj.downcast_ref::<i8>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<u8>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<i16>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<u16>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<i32>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<u32>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<isize>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<usize>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<i64>() {
        return *fdata as u32;
    } else if let Some(fdata) = Obj.downcast_ref::<u64>() {
        return *fdata as u32;
    }
    if ProcessFloat == true {
        if let Some(fdata) = Obj.downcast_ref::<f32>() {
            return *fdata as u32;
        } else if let Some(fdata) = Obj.downcast_ref::<f64>() {
            return *fdata as u32;
        }
    }
    return 0;
}

fn SRPRustToInt64(Obj: &Any, ProcessFloat: bool) -> u64 {
    if let Some(fdata) = Obj.downcast_ref::<bool>() {
        if *fdata {
            return 1;
        } else {
            return 0;
        }
    } else if let Some(fdata) = Obj.downcast_ref::<i8>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<u8>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<i16>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<u16>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<i32>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<u32>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<isize>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<usize>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<i64>() {
        return *fdata as u64;
    } else if let Some(fdata) = Obj.downcast_ref::<u64>() {
        return *fdata as u64;
    }
    if ProcessFloat == true {
        if let Some(fdata) = Obj.downcast_ref::<f32>() {
            return *fdata as u64;
        } else if let Some(fdata) = Obj.downcast_ref::<f64>() {
            return *fdata as u64;
        }
    }
    return 0;
}

fn SRPRustToLong(Obj: &Any, ProcessFloat: bool) -> i64 {
    if let Some(fdata) = Obj.downcast_ref::<bool>() {
        if *fdata {
            return 1;
        } else {
            return 0;
        }
    } else if let Some(fdata) = Obj.downcast_ref::<i8>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<u8>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<i16>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<u16>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<i32>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<u32>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<isize>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<usize>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<i64>() {
        return *fdata as i64;
    } else if let Some(fdata) = Obj.downcast_ref::<u64>() {
        return *fdata as i64;
    }
    if ProcessFloat == true {
        if let Some(fdata) = Obj.downcast_ref::<f32>() {
            return *fdata as i64;
        } else if let Some(fdata) = Obj.downcast_ref::<f64>() {
            return *fdata as i64;
        }
    }
    return 0;
}

fn SRPRustGetFloat(Obj: &Any) -> (f64, bool) {
    if let Some(fdata) = Obj.downcast_ref::<bool>() {
        if *fdata {
            return (1.0, true);
        } else {
            return (0.0, true);
        }
    } else if let Some(fdata) = Obj.downcast_ref::<i8>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<u8>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<i16>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<u16>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<i32>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<u32>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<isize>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<usize>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<i64>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<u64>() {
        return (*fdata as f64, true);
    }
    if let Some(fdata) = Obj.downcast_ref::<f32>() {
        return (*fdata as f64, true);
    } else if let Some(fdata) = Obj.downcast_ref::<f64>() {
        return (*fdata as f64, true);
    }
    return (0.0, false);
}

fn SRPRustToFloat(Obj: &Any) -> f64 {
    if let Some(fdata) = Obj.downcast_ref::<bool>() {
        if *fdata {
            return 1.0;
        } else {
            return 0.0;
        }
    } else if let Some(fdata) = Obj.downcast_ref::<i8>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<u8>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<i16>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<u16>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<i32>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<u32>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<isize>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<usize>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<i64>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<u64>() {
        return *fdata as f64;
    }
    if let Some(fdata) = Obj.downcast_ref::<f32>() {
        return *fdata as f64;
    } else if let Some(fdata) = Obj.downcast_ref::<f64>() {
        return *fdata as f64;
    }
    return 0.0;
}

fn SRPRustIsString(Obj: &Any) -> bool {
    if Obj.is::<String>() || Obj.is::<&str>() {
        return true;
    }
    return false;
}

/*-----------------------------------------------------------------------*/

fn RustPrintError(AlarmLevel: i32, Info: *const c_char) {
    unsafe {
        if StarRust_SRPControlInterface == 0 as *mut c_void {
            return;
        }
        starrust_SRPControl_ProcessError(
            StarRust_SRPControlInterface,
            AlarmLevel,
            CString::new("rust".to_owned()).unwrap().as_ptr(),
            0,
            Info,
        );
    }
}

/*-----------------------------------------------------------------------*/
fn SRPRustSetStr(Value: *mut c_char, FromAnsi: bool) -> String {
    let CharPtr: *mut c_char;

    if Value == 0 as *mut c_char {
        return "".to_owned();
    }
    if FromAnsi == true {
        CharPtr = unsafe { Star_SRPCS_AnsiToUTF8(StarRust_SRPCoreShellInterface, Value as *const i8, -1) as *mut c_char };
        if CharPtr == 0 as *mut c_char {
            RustPrintError(
                VSFAULT_WARNING,
                CString::new("coding to ansi fail...".to_owned())
                    .unwrap()
                    .as_ptr(),
            );
            return "".to_owned();
        }
        let ResultVal = unsafe { CStr::from_ptr(CharPtr) }
            .to_str()
            .unwrap()
            .to_owned();
        unsafe { Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, CharPtr as *mut c_void) };
        return ResultVal;
    }
    return unsafe { CStr::from_ptr(Value) }
        .to_str()
        .unwrap()
        .to_owned();
}

/*---does not contain any interior nul bytes--*/
fn SRPRustSetStrEx(Value: *mut c_char, StrLen: i32, FromAnsi: bool) -> String {
    let CharPtr: *mut c_char;
    let mut Retlen: i32 = 0;

    if Value == 0 as *mut c_char {
        return "".to_owned();
    }
    if FromAnsi == true {
        CharPtr = unsafe {
            Star_SRPCS_AnsiToUTF8Ex(StarRust_SRPCoreShellInterface, Value as *const i8, StrLen, &mut Retlen) as *mut c_char
        };
        if CharPtr == 0 as *mut c_char {
            RustPrintError(
                VSFAULT_WARNING,
                CString::new("coding to ansi fail...".to_owned())
                    .unwrap()
                    .as_ptr(),
            );
            return "".to_owned();
        }
        let ResultVal = unsafe { CStr::from_ptr(CharPtr) }
            .to_str()
            .unwrap()
            .to_owned();
        unsafe { Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, CharPtr as *mut c_void) };
        return ResultVal;
    }
    return unsafe { CStr::from_ptr(Value) }
        .to_str()
        .unwrap()
        .to_owned();
}

fn SRPRustGetStr(Obj: &Any, ToAnsi: bool) -> *mut c_char {
    let CharPtr: *mut c_char;

    //if ToAnsi == true {
        if let Some(ObjData) = Obj.downcast_ref::<String>() {
            CharPtr = unsafe {
                Star_SRPCS_UTF8ToAnsi(
                    StarRust_SRPCoreShellInterface,
                    CString::new(ObjData.clone()).unwrap().as_ptr() as *const i8,
                    -1
                ) as *mut c_char
            };
            return CharPtr;
        }else if let Some(ObjData) = Obj.downcast_ref::<&str>() {
            CharPtr = unsafe {
                Star_SRPCS_UTF8ToAnsi(
                    StarRust_SRPCoreShellInterface,
                    CString::new(ObjData.clone()).unwrap().as_ptr() as *const i8,
                    -1
                ) as *mut c_char
            };
            return CharPtr;
       }else{
           return 0 as *mut c_char;
       }          

    //} else {
    //    CharPtr = CString::new(Obj.clone()).unwrap().as_ptr() as *mut c_char;
    //}
    //return CharPtr;
}

fn SRPRustGetStrEx(Obj: &Any, ToAnsi: bool) -> (i32, *mut c_char) {
    let CharPtr: *mut c_char;

    //if ToAnsi == true {
        if let Some(ObjData) = Obj.downcast_ref::<String>() {
            let mut RetCharLength: i32 = 0;
            let len = ObjData.len() as i32;
            CharPtr = unsafe {
                Star_SRPCS_UTF8ToAnsiEx(
                    StarRust_SRPCoreShellInterface,
                    CString::new(ObjData.clone()).unwrap().as_ptr() as *const i8,
                    len,
                    &mut RetCharLength,
                ) as *mut c_char
            };
            return (RetCharLength, CharPtr);
        }else if let Some(ObjData) = Obj.downcast_ref::<&str>() {
            let mut RetCharLength: i32 = 0;
            let len = ObjData.len() as i32;
            CharPtr =
              unsafe {            
                Star_SRPCS_UTF8ToAnsiEx(
                    StarRust_SRPCoreShellInterface,
                    CString::new(ObjData.clone()).unwrap().as_ptr() as *const i8,
                    len,
                    &mut RetCharLength,
                ) as *mut c_char
            };
            return (RetCharLength, CharPtr);
        } else{
            return (0,0 as *mut c_char);
        }          
    //} else {
    //    let len = Obj.len() as i32;
    //    CharPtr = CString::new(Obj.clone()).unwrap().as_ptr() as *mut c_char;
    //    return (len, CharPtr);
    //}
}

fn STARRUST_SAFERELEASESTR(Obj: &Any, CharPtr: *const c_char) 
{
    unsafe{Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, CharPtr as *mut c_void);}
}

/*------------------------------------------------------------------------------------------------*/
/*------------------------------------------------------------------------------------------------*/
#[cfg(not(feature="star-sharelib"))]  
#[no_mangle]
pub extern "C" fn RustScriptTerm() {
    unsafe {
        if StarRust_ModuleInitFlag == VS_FALSE || StarRust_SRPControlInterface == 0 as *mut c_void {
            return;
        }
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|_info|{}));    
        let callresult = panic::catch_unwind(|| {               
            if G_SCRIPTTERMCALLBACK != Default_ScriptTermCallBack {
                G_SCRIPTTERMCALLBACK();
            }
        });
        panic::set_hook(old_hook);

        RustSRPClearSrvGroup(true);

        if G_MSGCALLBACK as *const c_void != Default_MsgCallBack as *const c_void {
            starrust_UnRegisterCallBackInfo(starrust_MsgCallBack as *mut c_void, 0);
            G_MSGCALLBACK = Default_MsgCallBack;
        }
        if G_DISPATCHREQUESTCALLBACK != Default_DispatchRequestCallBack {
            Star_SRPControl_UnRegDispatchRequest(
                StarRust_SRPControlInterface,
                starrust_SRPDispatchRequestCallBack as *mut c_void,
                0,
            );
            G_DISPATCHREQUESTCALLBACK = Default_DispatchRequestCallBack;
        }
    }
}

#[cfg(feature="star-sharelib")]  
#[no_mangle]
pub extern "C" fn RustScriptTerm() {
    unsafe {
        if StarRust_ModuleInitFlag == VS_FALSE || StarRust_SRPControlInterface == 0 as *mut c_void {
            return;
        }
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|_info|{}));    
        let callresult = panic::catch_unwind(|| {         
            if G_SCRIPTTERMCALLBACK != Default_ScriptTermCallBack {
                G_SCRIPTTERMCALLBACK();
            }else{
                ScriptTermCallBack();  /*--must defined for star-sharelib--*/
            }
        });
        panic::set_hook(old_hook);

        RustSRPClearSrvGroup(true);

        if G_MSGCALLBACK as *const c_void != Default_MsgCallBack as *const c_void {
            starrust_UnRegisterCallBackInfo(starrust_MsgCallBack as *mut c_void, 0);
            G_MSGCALLBACK = Default_MsgCallBack;
        }
        if G_DISPATCHREQUESTCALLBACK != Default_DispatchRequestCallBack {
            Star_SRPControl_UnRegDispatchRequest(
                StarRust_SRPControlInterface,
                starrust_SRPDispatchRequestCallBack as *mut c_void,
                0,
            );
            G_DISPATCHREQUESTCALLBACK = Default_DispatchRequestCallBack;
        }
    }
}

#[no_mangle]
pub extern "C" fn RustInitFromStarCore() {
    unsafe {
        if WEAKTABLEMUTEXINITFLAG == false {
            WEAKTABLEMUTEXINITFLAG = true;
            WEAKTABLEMUTEX = starrust_mutex_init();
        }
    }
}

#[no_mangle]
pub extern "C" fn RustMsgCallBack(
    ServiceGroupID: u32,
    uMsg: u32,
    wParam: usize,
    lParam: usize,
    IsProcessed: *mut VS_BOOL,
    Para: usize,
) -> usize {

    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(|_info|{}));
    
let callresult = panic::catch_unwind(|| {     
    unsafe {
        if G_MSGCALLBACK as *const c_void == Default_MsgCallBack as *const c_void {
            return 0;
        }
        if uMsg == MSG_VSDISPMSG || uMsg == MSG_VSDISPLUAMSG || uMsg == MSG_DISPMSG
            || uMsg == MSG_DISPLUAMSG || uMsg == MSG_EXIT || uMsg == MSG_HYPERLINK
            || uMsg == MSG_SETMANAGERCAPTION
        {
            //"(IIsI)"        
            let (Ret_IsProcessed, Ret_Result) = G_MSGCALLBACK(
                ServiceGroupID,
                uMsg,
                &SRPRustSetStr(starrust_ToPointer(wParam) as *mut c_char, true),
                &lParam,
            );
            if Ret_IsProcessed {
                (*IsProcessed) = VS_TRUE;
            } else {
                (*IsProcessed) = VS_FALSE;
            }
            return 0;
        } else if uMsg == MSG_ONTELNETSTRING {
            //"(IIss)"
            let (Ret_IsProcessed, Ret_Result) = G_MSGCALLBACK(
                ServiceGroupID,
                uMsg,
                &SRPRustSetStr(starrust_ToPointer(wParam) as *mut c_char, true),
                &SRPRustSetStr(starrust_ToPointer(lParam) as *mut c_char, true),
            );
            if Ret_IsProcessed {
                (*IsProcessed) = VS_TRUE;
            } else {
                (*IsProcessed) = VS_FALSE;
            }
            return 0;
        } else if uMsg == MSG_ONTELNETSTRING_PREEXECUTE {
            //"(IIss)"
            let (s, v) = G_MSGCALLBACK(
                ServiceGroupID,
                uMsg,
                &SRPRustSetStr(starrust_ToPointer(wParam) as *mut c_char, true),
                &SRPRustSetStr(starrust_ToPointer(lParam) as *mut c_char, true),
            );
            if s {
                if let Ok(Ret_Result) = v.downcast::<String>() {
                    let BasicSRPInterface = Star_SRPControl_QueryBasicInterface(
                        StarRust_SRPControlInterface,
                        ServiceGroupID,
                    );
                    if BasicSRPInterface != 0 as *mut c_void {
                        let RetStr = SRPRustGetStr(&Ret_Result, true);
                        if RetStr != 0 as *mut c_char {
                            let RetLen = vs_string_strlen(RetStr);
                            if RetLen == 0 {
                                (*IsProcessed) = VS_TRUE;
                            } else {
                                (*IsProcessed) = VS_FALSE;
                                if vs_string_strcmp(
                                    starrust_GetUWordPointer(wParam) as *const c_char,
                                    RetStr,
                                ) != 0
                                {
                                    Star_SRPBasic_Free(
                                        BasicSRPInterface,
                                        starrust_GetUWordPointer(wParam),
                                    );
                                    let NewStr = Star_SRPBasic_Malloc_Nor(
                                        BasicSRPInterface,
                                        (RetLen + 1) as i32,
                                    )
                                        as *mut c_char;
                                    vs_string_strcpy(NewStr, RetStr);
                                    starrust_SetUWordPointer(wParam, NewStr as *const c_void);
                                }
                            }
                            Star_SRPCS_FreeBuf(
                                StarRust_SRPCoreShellInterface,
                                RetStr as *mut c_void,
                            );
                        } else {
                            (*IsProcessed) = VS_FALSE;
                        }
                    } else {
                        (*IsProcessed) = VS_FALSE;
                    }
                    if BasicSRPInterface != 0 as *mut c_void {
                        Star_SRPBasic_Release(BasicSRPInterface);
                    }
                } else {
                    (*IsProcessed) = VS_FALSE;
                }
            } else {
                (*IsProcessed) = VS_FALSE;
            }
            return 0;
        } else if uMsg == MSG_APPEVENT {
            let (s, v) = G_MSGCALLBACK(
                ServiceGroupID,
                uMsg,
                &wParam,
                &SRPRustSetStr(starrust_ToPointer(lParam) as *mut c_char, true),
            );
            if s {
                (*IsProcessed) = VS_TRUE;
            } else {
                (*IsProcessed) = VS_FALSE;
            }
            return 0;
        }
        return 0;
    }
});
    panic::set_hook(old_hook);
    if callresult.is_err() {
        unsafe{(*IsProcessed) = VS_FALSE;}
        return 0;
    }else{    
        let vv = callresult.ok().expect("");
        return vv;
    }
}

#[no_mangle]
pub extern "C" fn RustSRPDispatchRequestCallBack() {
    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(|_info|{}));
    let callresult = panic::catch_unwind(|| {       
        unsafe {
            if G_DISPATCHREQUESTCALLBACK == Default_DispatchRequestCallBack {
                return;
            }
            G_DISPATCHREQUESTCALLBACK();
        }   
    });
    panic::set_hook(old_hook);
    if callresult.is_err() {
        return;
    }else{    
        return;
    }    
}

pub fn IsSrvGroupClassObject(Obj: &Any) -> bool {
    if let Some(fdata) = Obj.downcast_ref::<STARSRVGROUP>() {
        if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(fdata) {
            return true;
        }else{
            return false;
        }
    }
    return false;
}

pub fn IsStarServiceClassObject(Obj: &Any) -> bool {
    if let Some(fdata) = Obj.downcast_ref::<STARSERVICE>() {
        if let Some(ObjData) = STARGETRCREF_STARSERVICE!(fdata) {
            return true;
        }else{
            return false;
        }
    }
    return false;
}

pub fn IsStarObjectClassObject(Obj: &Any) -> bool {
    if let Some(fdata) = Obj.downcast_ref::<STAROBJECT>() {
        if let Some(ObjData) = STARGETRCREF_STAROBJECT!(fdata) {
            return true;
        }else{
            return false;
        }
    }
    return false;
}

pub fn IsStarParaPkgClassObject(Obj: &Any) -> bool {
    if let Some(fdata) = Obj.downcast_ref::<STARPARAPKG>() {
        if let Some(ObjData) = STARGETRCREF_STARPARAPKG!(fdata) {
            return true;
        }else{
            return false;
        }
    }
    return false;
}

pub fn IsStarBinBufClassObject(Obj: &Any) -> bool {
    if let Some(fdata) = Obj.downcast_ref::<STARBINBUF>() {
        if let Some(ObjData) = STARGETRCREF_STARBINBUF!(fdata) {
            return true;
        }else{
            return false;
        }
    }
    return false;
}

pub fn IsStarSXmlClassObject(Obj: &Any) -> bool {
    if let Some(fdata) = Obj.downcast_ref::<STARSXML>() {
        if let Some(ObjData) = STARGETRCREF_STARSXML!(fdata) {
            return true;
        }else{
            return false;
        }
    }
    return false;
}

pub fn RustObjectToSXml(Obj: &STARSXML) -> *mut c_void {
    if IsStarSXmlClassObject(Obj) == false {
        return 0 as *mut c_void;
    }
    if let Some(ObjData) = STARGETRCREF_STARSXML!(Obj) {
        return ObjData.ObjData.SXml as *mut c_void;
    } else {
        return 0 as *mut c_void;
    }
}

pub fn RustObjectToParaPkg(Obj: &STARPARAPKG) -> *mut c_void {
    if IsStarParaPkgClassObject(Obj) == false {
        return 0 as *mut c_void;
    }
    if let Some(ObjData) = STARGETRCREF_STARPARAPKG!(Obj) {
        return ObjData.ObjData.ParaPkg as *mut c_void;
    } else {
        return 0 as *mut c_void;
    }
}

pub fn RustObjectToBinBuf(Obj: &STARBINBUF) -> *mut c_void {
    if IsStarBinBufClassObject(Obj) == false {
        return 0 as *mut c_void;
    }
    if let Some(ObjData) = STARGETRCREF_STARBINBUF!(Obj) {
        return ObjData.ObjData.BinBuf as *mut c_void;
    } else {
        return 0 as *mut c_void;
    }
}

pub fn RustObjectToSRPObject(Obj: &Any) -> *mut c_void {
    if let Some(fdata) = Obj.downcast_ref::<STAROBJECT>() {
        if let Some(ObjData) = STARGETRCREF_STAROBJECT!(fdata) {
            let SRPInterface =
                GetSRPServiceInterfaceEx(ObjData.ObjData.ServiceGroupID, &ObjData.ObjData.ObjectID);
            if SRPInterface == 0 as *mut c_void {
                return 0 as *mut c_void;
            }
            return unsafe { Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjData.ObjectID.clone()) };
        } else {
            return 0 as *mut c_void;
        }
    }else{
        return 0 as *mut c_void;
    }
}

pub fn RustObjectToSRPServiceInterface(Obj: &STARSERVICE) -> *mut c_void {
    if IsStarServiceClassObject(Obj) == false {
        return 0 as *mut c_void;
    }
    if let Some(ObjData) = STARGETRCREF_STARSERVICE!(Obj) {
        return ObjData.ObjData.SRPInterface as *mut c_void;
    } else {
        return 0 as *mut c_void;
    }
}

pub fn RustObjectToSRPServiceInterface_byref(Ref: usize) -> *mut c_void {
    let Obj = SRefToRustObject_StarService(Ref);
    let mut STARGETRCMUT_temp = STARGETRCMUT_STARSERVICE!(&Obj); 
    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
        return ObjData.ObjData.SRPInterface as *mut c_void;
    } else {
        return 0 as *mut c_void;
    }
}

/*------------------------------------------------------------------------------------------------*/
/*------------------------------------------------------------------------------------------------*/
/*------------------------------------------------------------------------------------------------
  ---Global Functions
  ------------------------------------------------------------------------------------------------*/
fn SrvGroupClass_Data_dtor_before(fdata: &mut StarSrvGroupBody, ClearSrvGroupFlag: bool) {
    if fdata.BasicSRPInterface != 0 as *mut c_void && unsafe { StarRust_ModuleInitFlag } == VS_TRUE
    {
        if fdata.ObjectIndexTree != 0 as *mut c_void {
            let mut QueryRecord: VS_QUERYRECORD = VS_QUERYRECORD::new();
            let mut ObjectRefInSrvGroup = unsafe {
                Star_SRPBasic_QueryFirstIDKey(
                    fdata.BasicSRPInterface,
                    fdata.ObjectIndexTree,
                    &mut QueryRecord,
                    0 as *mut VS_UUID,
                )
            } as *mut StructOfObjectRefInSrvGroup;
            loop {
                if ObjectRefInSrvGroup == 0 as *mut StructOfObjectRefInSrvGroup {
                    break;
                }
                unsafe {
                    DeleteRustObjectAllRef((*ObjectRefInSrvGroup).RefItem);
                }
                unsafe {
                    starrust_FreeStructOfObjectRefInSrvGroup(ObjectRefInSrvGroup);
                }
                ObjectRefInSrvGroup = unsafe {
                    Star_SRPBasic_QueryNextIDKey(
                        fdata.BasicSRPInterface,
                        fdata.ObjectIndexTree,
                        &mut QueryRecord,
                        0 as *mut VS_UUID,
                    )
                } as *mut StructOfObjectRefInSrvGroup;
            }

            if ClearSrvGroupFlag == true {
                unsafe {
                    Star_SRPBasic_DestoryIndex(fdata.BasicSRPInterface, fdata.ObjectIndexTree);
                }
                fdata.ObjectIndexTree = 0 as *mut c_void;
            } else {
                unsafe {
                    Star_SRPBasic_DelAllKey(fdata.BasicSRPInterface, fdata.ObjectIndexTree);
                }
            }
        }

        if ClearSrvGroupFlag == true {
            unsafe {
                Star_SRPBasic_UnRegObjectIDChangeNotify(
                    fdata.BasicSRPInterface,
                    starrust_ObjectIDChangeNotify as *const c_void,
                    fdata.RefItem,
                );
            }
            unsafe {
                Star_SRPBasic_UnRegObjectFreeNotify(
                    fdata.BasicSRPInterface,
                    starrust_ObjectFreeNotify as *const c_void,
                    fdata.RefItem,
                );
            }
        }
        //--SRPInterfaceItem
        for SRPInterfaceItem in fdata.SRPInterfaceItemRoot.iter_mut() {
            DeleteRustObjectAllRef(SRPInterfaceItem.ServiceObject);
        }
        fdata.SRPInterfaceItemRoot.clear();

        if ClearSrvGroupFlag == true {
            unsafe {
                Star_SRPBasic_Release(fdata.BasicSRPInterface);
            }
            fdata.BasicSRPInterface = 0 as *mut c_void;
        }
    }
    if ClearSrvGroupFlag == true {
        fdata.ServiceGroupID = VS_INVALID_SERVICEGROUPID;
    }
}

struct StructOfRustSrvGroupIndex {
    RustSrvGroupObject: usize,
    ServiceGroupID: u32,
}
impl StructOfRustSrvGroupIndex {
    pub fn new() -> StructOfRustSrvGroupIndex {
        StructOfRustSrvGroupIndex {
            RustSrvGroupObject: 0 as usize,
            ServiceGroupID: 0 as u32,
        }
    }
}

static mut RUSTSRVGROUPINDEXROOT: *mut Vec<StructOfRustSrvGroupIndex> = 0 as *mut _;

fn RustSRPGetSrvGroup(ServiceGroupID: u32, BasicSRPInterface: *const c_void) -> STARSRVGROUP {
    unsafe {
        if RUSTSRVGROUPINDEXROOT == 0 as *mut _ {
            RUSTSRVGROUPINDEXROOT = Box::into_raw(Box::new(Vec::new()));
        }
        let RustSrvGroupIndexRoot =
            &mut *(RUSTSRVGROUPINDEXROOT as *mut Vec<StructOfRustSrvGroupIndex>);
        for RustSrvGroupIndexVal in RustSrvGroupIndexRoot.iter_mut() {
            if RustSrvGroupIndexVal.ServiceGroupID == ServiceGroupID {
                return SRefToRustObject_StarSrvGroup(RustSrvGroupIndexVal.RustSrvGroupObject);
            }
        }
        let newobj = ToStarSrvGroup(ServiceGroupID, Star_SRPBasic_Dup(BasicSRPInterface));

        let mut RustSrvGroupIndex = StructOfRustSrvGroupIndex::new();
        RustSrvGroupIndex.ServiceGroupID = ServiceGroupID;

        let RefItem = STARSRVGROUP_RefItem(&newobj);
        RustSrvGroupIndex.RustSrvGroupObject = NewRustObjectRefStrong_StarSrvGroup(&newobj, RefItem);

        RustSrvGroupIndexRoot.push(RustSrvGroupIndex);
        return newobj;
    }
}

fn RustSRPDeleteSrvGroup(ServiceGroupID: u32) {
    unsafe {
        if RUSTSRVGROUPINDEXROOT == 0 as *mut _ {
            RUSTSRVGROUPINDEXROOT = Box::into_raw(Box::new(Vec::new()));
        }
        let RustSrvGroupIndexRoot =
            &mut *(RUSTSRVGROUPINDEXROOT as *mut Vec<StructOfRustSrvGroupIndex>);

        for i in 0..RustSrvGroupIndexRoot.len() {
            if RustSrvGroupIndexRoot[i].ServiceGroupID == ServiceGroupID {
                DeleteRustObjectAllRef(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
                RustSrvGroupIndexRoot.remove(i);
                return;
            }
        }
        return;
    }
}

fn RustSRPGetBasicSRPInterface(ServiceGroupID: u32) -> *mut c_void {
    unsafe {
        if RUSTSRVGROUPINDEXROOT == 0 as *mut _ {
            RUSTSRVGROUPINDEXROOT = Box::into_raw(Box::new(Vec::new()));
        }
        let RustSrvGroupIndexRoot =
            &mut *(RUSTSRVGROUPINDEXROOT as *mut Vec<StructOfRustSrvGroupIndex>);

        for i in 0..RustSrvGroupIndexRoot.len() {
            if RustSrvGroupIndexRoot[i].ServiceGroupID == ServiceGroupID {
                let Obj =
                    SRefToRustObject_StarSrvGroup(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
                let STARGETRCMUT_Temp = STARGETRCMUT_STARSRVGROUP_ToMut(Obj.as_ref()).borrow();
                //if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(Obj) {
                if let Some(ObjData) = STARGETRCMUT_Temp.as_ref() {
                    return ObjData.ObjData.BasicSRPInterface as *mut c_void;
                } else {
                    return 0 as *mut c_void;
                }
            }
        }
        return 0 as *mut c_void;
    }
}

fn RustSRPQuerySrvGroup(ServiceGroupID: u32) -> STARSRVGROUP {
    unsafe {
        if RUSTSRVGROUPINDEXROOT == 0 as *mut _ {
            RUSTSRVGROUPINDEXROOT = Box::into_raw(Box::new(Vec::new()));
        }
        let RustSrvGroupIndexRoot =
            &mut *(RUSTSRVGROUPINDEXROOT as *mut Vec<StructOfRustSrvGroupIndex>);

        for i in 0..RustSrvGroupIndexRoot.len() {
            if RustSrvGroupIndexRoot[i].ServiceGroupID == ServiceGroupID {
                return SRefToRustObject_StarSrvGroup(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
            }
        }
        return STARRC!(None);
    }
}

fn RustSRPClearSrvGroup(ClearSrvGroupFlag: bool) {
    unsafe {
        if RUSTSRVGROUPINDEXROOT == 0 as *mut _ {
            RUSTSRVGROUPINDEXROOT = Box::into_raw(Box::new(Vec::new()));
        }
        let RustSrvGroupIndexRoot =
            &mut *(RUSTSRVGROUPINDEXROOT as *mut Vec<StructOfRustSrvGroupIndex>);

        let mut ZeroRustSrvGroupIndex: i32 = -1;

        for i in 0..RustSrvGroupIndexRoot.len() {
            if ClearSrvGroupFlag == true {
                {
                    let mut s =
                        SRefToRustObject_StarSrvGroup(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&s); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                        SrvGroupClass_Data_dtor_before(&mut ObjData.ObjData, ClearSrvGroupFlag);
                    }
                }
                DeleteRustObjectAllRef(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
                RustSrvGroupIndexRoot[i].RustSrvGroupObject = 0;
            } else {
                if RustSrvGroupIndexRoot[i].ServiceGroupID != 0 {
                    {
                        let mut s =
                            SRefToRustObject_StarSrvGroup(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
                        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&s);
                        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                            SrvGroupClass_Data_dtor_before(&mut ObjData.ObjData, true);
                        }
                    }
                    DeleteRustObjectAllRef(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
                    RustSrvGroupIndexRoot[i].RustSrvGroupObject = 0;
                } else {
                    {
                        let mut s =
                            SRefToRustObject_StarSrvGroup(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
                        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&s); 
                        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
                            SrvGroupClass_Data_dtor_before(&mut ObjData.ObjData, true);
                        }
                    }
                    ZeroRustSrvGroupIndex = i as i32;
                }
            }
        }
        if ZeroRustSrvGroupIndex < 0 {
            RustSrvGroupIndexRoot.clear();
        } else {
            let ZeroRustSrvGroupIndex =
                RustSrvGroupIndexRoot.remove(ZeroRustSrvGroupIndex as usize);
            RustSrvGroupIndexRoot.clear();
            RustSrvGroupIndexRoot.push(ZeroRustSrvGroupIndex);
        }
        return;
    }
}

fn RustSRPClearSrvGroupWithID(ServiceGroupID: u32) {
    unsafe {
        if RUSTSRVGROUPINDEXROOT == 0 as *mut _ {
            RUSTSRVGROUPINDEXROOT = Box::into_raw(Box::new(Vec::new()));
        }
        let RustSrvGroupIndexRoot =
            &mut *(RUSTSRVGROUPINDEXROOT as *mut Vec<StructOfRustSrvGroupIndex>);

        for i in 0..RustSrvGroupIndexRoot.len() {
            if RustSrvGroupIndexRoot[i].ServiceGroupID == ServiceGroupID {
                DeleteRustObjectAllRef(RustSrvGroupIndexRoot[i].RustSrvGroupObject);
                RustSrvGroupIndexRoot.remove(i);
                return;
            }
        }
        return;
    }
}

fn RustSRPSrvGroup_CheckServiceValid(fdata: &mut StarSrvGroupBody) {
    for i in 0..fdata.SRPInterfaceItemRoot.len() {
        let SRPInterface =
            RustObjectToSRPServiceInterface_byref(fdata.SRPInterfaceItemRoot[i].ServiceObject);
        if SRPInterface == 0 as *mut c_void
            || unsafe { Star_SRPI_IsValid(SRPInterface) } == VS_FALSE
        {
            DeleteRustObjectAllRef(fdata.SRPInterfaceItemRoot[i].ServiceObject);
            fdata.SRPInterfaceItemRoot.remove(i);
            return;
        }
    }
    return;
}

fn RustSRPQueryServiceByServiceID(
    fdata: &mut StarSrvGroupBody,
    ServiceID: *const VS_UUID,
) -> STARSERVICE {
    RustSRPSrvGroup_CheckServiceValid(fdata);
    for i in 0..fdata.SRPInterfaceItemRoot.len() {
        if unsafe { starrust_uuidisequal(&fdata.SRPInterfaceItemRoot[i].ServiceID, ServiceID) }
            == VS_TRUE
        {
            return SRefToRustObject_StarService(fdata.SRPInterfaceItemRoot[i].ServiceObject);
        }
    }
    return STARRC!(None);
}

fn RustSRPQueryServiceByServiceName(
    fdata: &mut StarSrvGroupBody,
    ServiceName: *const c_char,
) -> STARSERVICE {
    RustSRPSrvGroup_CheckServiceValid(fdata);
    for i in 0..fdata.SRPInterfaceItemRoot.len() {
        let SRPInterface =
            RustObjectToSRPServiceInterface_byref(fdata.SRPInterfaceItemRoot[i].ServiceObject);
        unsafe {
            if SRPInterface != 0 as *mut c_void && Star_SRPI_IsValid(SRPInterface) == VS_TRUE
                && vs_string_strcmp(ServiceName, Star_SRPI_GetServiceName(SRPInterface)) == 0
            {
                return SRefToRustObject_StarService(fdata.SRPInterfaceItemRoot[i].ServiceObject);
            }
        }
    }
    return STARRC!(None);
}

fn RustSRPSetObject(
    ServiceGroupID: u32,
    ObjectID: *const VS_UUID,
    Object: &STAROBJECT,
    IncreaseRefCount: bool,
) {
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            let selfval = &ObjData.ObjData;
            let mut ObjectRefInSrvGroup = Star_SRPBasic_FindIDKey(
                selfval.BasicSRPInterface,
                selfval.ObjectIndexTree,
                ObjectID as *mut VS_UUID,
            ) as *mut StructOfObjectRefInSrvGroup;
            if ObjectRefInSrvGroup == 0 as *mut StructOfObjectRefInSrvGroup {
                let mut NewObjectRefInSrvGroup = starrust_MallocStructOfObjectRefInSrvGroup();
                /*---must not change the reference--*/
                let obj_refitem = STAROBJECT_RefItem(Object);
                DeleteRustObjectAllRef(obj_refitem);
                if IncreaseRefCount == true {
                    (*NewObjectRefInSrvGroup).RefItem = NewRustObjectRefStrong_StarObject(Object,obj_refitem);
                    (*NewObjectRefInSrvGroup).IncreaseRef = true;
                } else {
                    (*NewObjectRefInSrvGroup).RefItem = NewRustObjectRef_StarObject(Object,obj_refitem);
                    (*NewObjectRefInSrvGroup).IncreaseRef = false;
                }
                Star_SRPBasic_InsertIDKey(
                    selfval.BasicSRPInterface,
                    selfval.ObjectIndexTree,
                    ObjectID as *mut VS_UUID,
                    NewObjectRefInSrvGroup as *mut i8,
                );
            } else {
                if IncreaseRefCount == true {
                    if (*ObjectRefInSrvGroup).IncreaseRef == false {
                        (*ObjectRefInSrvGroup).RefItem =
                            NewRustObjectRefStrong_StarObject(Object, (*ObjectRefInSrvGroup).RefItem);
                        (*ObjectRefInSrvGroup).IncreaseRef = true;
                    }
                }
            }
        } else {
            return;
        }
    }
}

fn RustSRPSUnlockObject(ServiceGroupID: u32, ObjectID: *mut VS_UUID, Object: &mut STAROBJECT) 
{
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            let selfval = &ObjData.ObjData;
            let mut ObjectRefInSrvGroup = Star_SRPBasic_FindIDKey(
                selfval.BasicSRPInterface,
                selfval.ObjectIndexTree,
                ObjectID as *mut VS_UUID,
            ) as *mut StructOfObjectRefInSrvGroup;
            if ObjectRefInSrvGroup == 0 as *mut StructOfObjectRefInSrvGroup {
                return;
            }
	        if (*ObjectRefInSrvGroup).IncreaseRef == true {
		        DeleteRustObjectAllRef((*ObjectRefInSrvGroup).RefItem);
		        (*ObjectRefInSrvGroup).RefItem = NewRustObjectRef_StarObject(Object, (*ObjectRefInSrvGroup).RefItem);
		        (*ObjectRefInSrvGroup).IncreaseRef = false;
            }
	    } else{
            return;
        }  
    }
}

fn RustSRPGetObject(ServiceGroupID: u32, ObjectID: *mut VS_UUID) -> STAROBJECT 
{
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            let selfval = &ObjData.ObjData;
            let mut ObjectRefInSrvGroup = Star_SRPBasic_FindIDKey(
                selfval.BasicSRPInterface,
                selfval.ObjectIndexTree,
                ObjectID as *mut VS_UUID,
            ) as *mut StructOfObjectRefInSrvGroup;
            if ObjectRefInSrvGroup == 0 as *mut StructOfObjectRefInSrvGroup {
                return STARRC!(None);
            }
	        return SRefToRustObject_StarObject((*ObjectRefInSrvGroup).RefItem);
        }else{
            return STARRC!(None); 
        }
    }
}

fn GetRustObjectFromSRPObject(SRPObject: *mut c_void, SRPInterface: *const c_void)  -> STAROBJECT
{
	if SRPObject == 0 as *mut c_void {
		return STARRC!(None); 
	}
    let mut ObjectID : VS_UUID = VS_UUID::new();
	unsafe{Star_SRPI_GetID(SRPInterface, SRPObject, &mut ObjectID);}
	return RustSRPGetObject(unsafe{Star_SRPI_GetServiceGroupID(SRPInterface)}, &mut ObjectID);
}

fn RustSRPIsObjectGlobalRef(ServiceGroupID: u32, ObjectID: *mut VS_UUID) -> bool 
{
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            let selfval = &ObjData.ObjData;
            let mut ObjectRefInSrvGroup = Star_SRPBasic_FindIDKey(
                selfval.BasicSRPInterface,
                selfval.ObjectIndexTree,
                ObjectID as *mut VS_UUID,
            ) as *mut StructOfObjectRefInSrvGroup;
            if ObjectRefInSrvGroup == 0 as *mut StructOfObjectRefInSrvGroup {
                return false;
            }
            return (*ObjectRefInSrvGroup).IncreaseRef;
        }else{
            return false;
        }
    }
}

fn RustSRPClearObject(ServiceGroupID: u32, ObjectID: *mut VS_UUID) 
{
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            let selfval = &ObjData.ObjData;
            let mut ObjectRefInSrvGroup = Star_SRPBasic_FindIDKey(
                selfval.BasicSRPInterface,
                selfval.ObjectIndexTree,
                ObjectID as *mut VS_UUID,
            ) as *mut StructOfObjectRefInSrvGroup;
            if ObjectRefInSrvGroup != 0 as *mut StructOfObjectRefInSrvGroup {
                DeleteRustObjectAllRef((*ObjectRefInSrvGroup).RefItem);
            }
            return;
        }else{
            return;
        }
    }
}

fn GetSRPServiceInterface(ServiceGroupID: u32,Object: *const c_void) -> *mut c_void 
{
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            let mut selfval = &mut ObjData.ObjData;
	        if selfval.SRPInterfaceItemRoot.len() == 0 {
		        return 0 as *mut c_void;
	        }
	        RustSRPSrvGroup_CheckServiceValid(selfval);
	        if selfval.SRPInterfaceItemRoot.len() == 0 {
		        return 0 as *mut c_void;
	        }
            for i in 0..selfval.SRPInterfaceItemRoot.len(){
 		        let SRPInterface = RustObjectToSRPServiceInterface_byref(selfval.SRPInterfaceItemRoot[i].ServiceObject);
		        if Object == 0 as *const c_void || SRPInterface == 0 as *mut c_void || (Star_SRPI_IsValid(SRPInterface) == VS_TRUE && Star_SRPI_IsThisService(SRPInterface, Object as *mut c_void) == VS_TRUE) {
			        return SRPInterface;
		        }
	        }
	        let mut SRPInterface = RustObjectToSRPServiceInterface_byref(selfval.SRPInterfaceItemRoot[0].ServiceObject);
	        SRPInterface = Star_SRPI_GetSRPInterface(SRPInterface, Object as *mut c_void);
	        ToStarService(selfval, SRPInterface);
	        return SRPInterface;
        }else{
            return 0 as *mut c_void;
        }
    }
}


fn GetSRPServiceInterfaceEx(ServiceGroupID: u32, ObjectID: *const VS_UUID) -> *mut c_void {
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_Temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp);
        if let Some(ObjData) = STARGETRCMUT_Temp.as_mut() {
            let mut selfval = &mut ObjData.ObjData;
	        if selfval.SRPInterfaceItemRoot.len() == 0 {
		        return 0 as *mut c_void;
	        }
	        RustSRPSrvGroup_CheckServiceValid(selfval);
	        if selfval.SRPInterfaceItemRoot.len() == 0 {
		        return 0 as *mut c_void;
	        }
            for i in 0..selfval.SRPInterfaceItemRoot.len(){
 		        let SRPInterface = RustObjectToSRPServiceInterface_byref(selfval.SRPInterfaceItemRoot[i].ServiceObject);
		        if ObjectID == 0 as *const VS_UUID || SRPInterface == 0 as *mut c_void || (Star_SRPI_IsValid(SRPInterface) == VS_TRUE && Star_SRPI_IsThisServiceEx(SRPInterface, ObjectID as *mut VS_UUID) == VS_TRUE) {
			        return SRPInterface;
		        }
	        }
	        let mut SRPInterface = RustObjectToSRPServiceInterface_byref(selfval.SRPInterfaceItemRoot[0].ServiceObject);
	        SRPInterface = Star_SRPI_GetSRPInterfaceEx(SRPInterface, ObjectID as *mut VS_UUID);
	        ToStarService(selfval, SRPInterface);
	        return SRPInterface;
        }else{
            return 0 as *mut c_void;
        }
    }
}

fn RustSRPQueryService(ServiceGroupID: u32, Object: *const c_void) -> STARSERVICE 
{
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            let mut selfval = &mut ObjData.ObjData;
	        if selfval.SRPInterfaceItemRoot.len() == 0 {
		        return STARRC!(None);
	        }
	        RustSRPSrvGroup_CheckServiceValid(selfval);
	        if selfval.SRPInterfaceItemRoot.len() == 0 {
		        return STARRC!(None);
	        }
            for i in 0..selfval.SRPInterfaceItemRoot.len(){
 		        let SRPInterface = RustObjectToSRPServiceInterface_byref(selfval.SRPInterfaceItemRoot[i].ServiceObject);
		        if SRPInterface == 0 as *mut c_void || Star_SRPI_IsValid(SRPInterface) == VS_FALSE {
			        return STARRC!(None);
		        }
                if Object == 0 as *const c_void || Star_SRPI_IsThisService(SRPInterface, Object as *mut c_void) == VS_TRUE {
                    return SRefToRustObject_StarService(selfval.SRPInterfaceItemRoot[i].ServiceObject);
                }
	        }
	        let mut SRPInterface = RustObjectToSRPServiceInterface_byref(selfval.SRPInterfaceItemRoot[0].ServiceObject);
	        SRPInterface = Star_SRPI_GetSRPInterface(SRPInterface, Object as *mut c_void);
	        return ToStarService(selfval, SRPInterface);
        }else{
            return STARRC!(None);
        }
    }
}

fn RustSRPQueryServiceEx(ServiceGroupID: u32, ObjectID: *const VS_UUID) -> STARSERVICE 
{
    unsafe {
        let ObjectTemp = RustSRPQuerySrvGroup(ServiceGroupID);
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
            let mut selfval = &mut ObjData.ObjData;
	        if selfval.SRPInterfaceItemRoot.len() == 0 {
		        return STARRC!(None);
	        }
	        RustSRPSrvGroup_CheckServiceValid(selfval);
	        if selfval.SRPInterfaceItemRoot.len() == 0 {
		        return STARRC!(None);
	        }
            for i in 0..selfval.SRPInterfaceItemRoot.len(){
 		        let SRPInterface = RustObjectToSRPServiceInterface_byref(selfval.SRPInterfaceItemRoot[i].ServiceObject);
		        if SRPInterface == 0 as *mut c_void || Star_SRPI_IsValid(SRPInterface) == VS_FALSE {
			        return STARRC!(None);
		        }
                if ObjectID == 0 as *const VS_UUID || Star_SRPI_IsThisServiceEx(SRPInterface, ObjectID as *mut VS_UUID) == VS_TRUE {
                    return SRefToRustObject_StarService(selfval.SRPInterfaceItemRoot[i].ServiceObject);
                }
	        }
	        let mut SRPInterface = RustObjectToSRPServiceInterface_byref(selfval.SRPInterfaceItemRoot[0].ServiceObject);
	        SRPInterface = Star_SRPI_GetSRPInterfaceEx(SRPInterface, ObjectID as *mut VS_UUID);
	        return ToStarService(selfval, SRPInterface);
        }else{
            return STARRC!(None);
        }
    }
}

fn RustClearStarCoreContext(selfval: &mut Any) {
    unsafe{
    	if IsStarServiceClassObject(selfval) == true { 
            if let Some(s) = selfval.downcast_mut::<STARSERVICE>(){
                let mut STARGETRCMUT_temp = STARGETRCMUT_STARSERVICE!(s); 
                if let Some(ObjData) = STARGETRCMUT_temp.as_mut()  {
		            let fdata = &mut ObjData.ObjData;
		            fdata.IsClearedByStarCore = true;
    		        if StarRust_ModuleInitFlag == VS_TRUE {
	    		        if fdata.SRPInterface != 0 as *const c_void {
		    		        Star_SRPI_Release(fdata.SRPInterface);
			            }
			            fdata.SRPInterface = 0 as *const c_void;
    		        }
	    	        return;
	            }else{
                    return;
                }
            }else{
                return;
            }
        }
    	if IsStarParaPkgClassObject(selfval) == true { 
            if let Some(s) = selfval.downcast_mut::<STARPARAPKG>(){
                let mut STARGETRCMUT_temp = STARGETRCMUT_STARPARAPKG!(s); 
                if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
		            let fdata = &mut ObjData.ObjData;
		            fdata.IsClearedByStarCore = true;
            		if fdata.FreeFlag == true && fdata.ParaPkg != 0 as *const c_void {
			            if StarRust_ModuleInitFlag == VS_TRUE {
            				Star_SRPParaPkg_Release(fdata.ParaPkg);
			            }
            			fdata.ParaPkg =0 as *const c_void;
            		}
	    	        return;
	            }else{
                    return;
                }
            }else{
                return;
            }
        }        
    	if IsStarBinBufClassObject(selfval) == true { 
            if let Some(s) = selfval.downcast_mut::<STARBINBUF>(){
                let mut STARGETRCMUT_temp = STARGETRCMUT_STARBINBUF!(s); 
                if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
		            let fdata = &mut ObjData.ObjData;
		            fdata.IsClearedByStarCore = true;
            		if fdata.FreeFlag == true && fdata.BinBuf != 0 as *const c_void {
			            if StarRust_ModuleInitFlag == VS_TRUE {
            				Star_SRPBinBuf_Release(fdata.BinBuf);
			            }
                    }
            		fdata.BinBuf =0 as *const c_void;
	    	        return;
	            }else{
                    return;
                }
            }else{
                return;
            }
        }
    	if IsStarSXmlClassObject(selfval) == true { 
            if let Some(s) = selfval.downcast_mut::<STARSXML>(){
                let mut STARGETRCMUT_temp = STARGETRCMUT_STARSXML!(s); 
                if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
		            let fdata = &mut ObjData.ObjData;
		            fdata.IsClearedByStarCore = true;
            		if fdata.FreeFlag == true && fdata.SXml != 0 as *const c_void {
			            if StarRust_ModuleInitFlag == VS_TRUE {
            				Star_SRPSXML_Release(fdata.SXml);
			            }
                    }
            		fdata.SXml =0 as *const c_void;
	    	        return;
	            }else{
                    return;
                }
            }else{
                return;
            }
        }
    	if IsStarObjectClassObject(selfval) == true { 
            if let Some(s) = selfval.downcast_mut::<STAROBJECT>(){
                let mut ServiceGroupID:u32;
                let mut ObjectID:VS_UUID;
                {
                    let mut STARGETRCMUT_temp = STARGETRCMUT_STAROBJECT!(s); 
                    if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
		                let fdata = &mut ObjData.ObjData;
		                fdata.IsClearedByStarCore = true;

                        ServiceGroupID = fdata.ServiceGroupID;
                        ObjectID = fdata.ObjectID.clone();
    		            //RustSRPSUnlockObject(fdata.ServiceGroupID, &mut fdata.ObjectID, s);
	    	            //RustSRPClearObject(fdata.ServiceGroupID, &mut fdata.ObjectID);
	        	        //return;
	                }else{
                        return;
                    }
                }
		        RustSRPSUnlockObject(ServiceGroupID, &mut ObjectID, s);
		        RustSRPClearObject(ServiceGroupID, &mut ObjectID);                
                return;
            }else{
                return;
            }
        }      
    }
}

#[no_mangle]
pub extern "C" fn RustFreeScriptObject(RefItem: usize)
{
	let mut s = SRefToRustObject_StarObject(RefItem);
    RustClearStarCoreContext(&mut s);
}

#[no_mangle]
pub extern "C" fn RustObjectFreeNotify(Object: *const c_void, RefItem: usize) 
{
    unsafe {    
    	if StarRust_ModuleInitFlag == VS_FALSE {
	    	return;
    	}
        let ObjectTemp = SRefToRustObject_StarSrvGroup(RefItem);
        let mut ObjectID: VS_UUID = VS_UUID::new();

        let ObjData : StarSrvGroupBody;
        if let Some(fbody) = STARGETRCREF_STARSRVGROUP!(&ObjectTemp) {    
            ObjData = fbody.ObjData.clone();
        }else{
            return;
        }
//        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(&ObjectTemp); 
//        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
        	Star_SRPBasic_GetID(ObjData.BasicSRPInterface, Object as *mut c_void, &mut ObjectID);
            let ObjectRefInSrvGroup = Star_SRPBasic_FindIDKey(
                ObjData.BasicSRPInterface,
                ObjData.ObjectIndexTree,
                &mut ObjectID,
            ) as *mut StructOfObjectRefInSrvGroup;
        	if ObjectRefInSrvGroup == 0 as *mut StructOfObjectRefInSrvGroup {
        		return;
        	}
	        let mut S = SRefToRustObject_StarObject((*ObjectRefInSrvGroup).RefItem);
	        if IsStarObjectClassObject(&mut S) == true {
		        RustClearStarCoreContext(&mut S);
	        }
//        }else{
//            return;
//        }
    }
}

#[no_mangle]
pub extern "C" fn RustObjectIDChangeNotify(
    Object: *const c_void,
    Para: usize,
    NewObjectID: *mut VS_UUID,
) {
    unsafe {    
    	if StarRust_ModuleInitFlag == VS_FALSE {
	    	return;
    	}
        let ObjectTemp = SRefToRustObject_StarSrvGroup(Para);
        let mut ObjectID: VS_UUID = VS_UUID::new();
        let mut STARGETRCMUT_temp =  STARGETRCMUT_STARSRVGROUP!(&ObjectTemp);
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {
        	Star_SRPBasic_GetID(ObjData.ObjData.BasicSRPInterface, Object as *mut c_void, &mut ObjectID);
            let mut ObjectRefInSrvGroup = Star_SRPBasic_DelIDKey(
                ObjData.ObjData.BasicSRPInterface,
                ObjData.ObjData.ObjectIndexTree,
                &mut ObjectID,
            ) as *mut StructOfObjectRefInSrvGroup;
        	if ObjectRefInSrvGroup == 0 as *mut StructOfObjectRefInSrvGroup {
        		return;
        	}            
        	Star_SRPBasic_InsertIDKey(ObjData.ObjData.BasicSRPInterface, ObjData.ObjData.ObjectIndexTree, NewObjectID, ObjectRefInSrvGroup as *mut i8);
	        let mut S = SRefToRustObject_StarObject((*ObjectRefInSrvGroup).RefItem);
	        if IsStarObjectClassObject(&mut S) == false {
                return;
            }
            let mut STARGETRCMUT_temp_s = STARGETRCMUT_STAROBJECT!(&S);
            if let Some(b) = STARGETRCMUT_temp_s.as_mut() {
                b.ObjData.ObjectID = (*NewObjectID).clone();
            }else{

            }
            return;
	    }else{
	        return;
        }
    }
}

/*------------------------------------------------------------------------------------------------*/
/*  end of global functions */
/*------------------------------------------------------------------------------------------------*/

pub mod starrust {
    pub use super::*;

    /*------------------------------------------------------------------------------------------------*/
    pub fn InitSimple(
        In_ServiceName: &Any,
        In_ServicePass: &Any,
        ClientPortNumber: i32,
        WebPortNumber: i32,
        DependService: &[&Any],
    ) -> STARSERVICE {
        unsafe {
            if starrust_starcore_Load() != 0 {
                println!("{}","please download and install cle from http://www.srplab.com/en/files/products.html");
                panic!("starcore init fail");
            }
            starrust_starcore_Init(
                VS_TRUE,
                VS_FALSE,
                VS_FALSE,
                VS_TRUE,
                CString::new("").unwrap().as_ptr() as *const c_char,
                0,
                CString::new("").unwrap().as_ptr() as *const c_char,
                ClientPortNumber,
            );
            let BasicSRPInterface =
                Star_SRPControl_QueryBasicInterface(StarRust_SRPControlInterface, 0);
            let ServiceName = SRPRustGetStr(In_ServiceName, false);
            let ServicePass = SRPRustGetStr(In_ServicePass, false);

            for num in 0..DependService.len() {
                let DependServiceName = SRPRustGetStr(DependService[num], false);
                if vs_string_strlen(DependServiceName) != 0
                    && Star_SRPBasic_ImportService(BasicSRPInterface, DependServiceName, VS_TRUE)
                        == VS_FALSE
                {
                    Star_SRPBasic_Release(BasicSRPInterface);
                    panic!(format!{"import depend service {:?} fail", DependService[num]});
                }
                STARRUST_SAFERELEASESTR(DependService[num], DependServiceName);
            }

            if Star_SRPBasic_CreateService(
                BasicSRPInterface,
                CString::new("").unwrap().as_ptr() as *const c_char,
                ServiceName,
                0 as *mut VS_UUID,
                ServicePass,
                5,
                10240,
                10240,
                10240,
                10240,
                10240,
            ) == VS_FALSE
            {
                Star_SRPBasic_Release(BasicSRPInterface);
                panic!(format!{"create service [{:?}] fail", In_ServiceName});
            }
            let SRPInterface = Star_SRPBasic_GetSRPInterface(
                BasicSRPInterface,
                ServiceName,
                CString::new("root").unwrap().as_ptr() as *const c_char,
                ServicePass,
            );

            STARRUST_SAFERELEASESTR(In_ServiceName, ServiceName);
            STARRUST_SAFERELEASESTR(In_ServicePass, ServicePass);

            if SRPInterface == 0 as *mut c_void {
                Star_SRPBasic_Release(BasicSRPInterface);
                return STARRC!(None);
            }
            if WebPortNumber != 0 {
                Star_SRPBasic_SetWebServerPort(
                    BasicSRPInterface,
                    CString::new("").unwrap().as_ptr() as *const c_char,
                    WebPortNumber as u16,
                    100,
                    2048,
                );
            }
            if WEAKTABLEMUTEXINITFLAG == false {
                WEAKTABLEMUTEXINITFLAG = true;
                WEAKTABLEMUTEX = starrust_mutex_init();
            }

            let SrvGroupVal = RustSRPGetSrvGroup(0, BasicSRPInterface);
            let mut STARGETRCMUT_Temp = STARGETRCMUT_STARSRVGROUP!(&SrvGroupVal);
            if let Some(sss) = STARGETRCMUT_Temp.as_mut() {
                let ObjectTemp = ToStarService(
                    &mut sss.ObjData,
                    SRPInterface,
                );
                Star_SRPBasic_Release(BasicSRPInterface);

                return ObjectTemp;
            }else{
                return STARRC!(None);
            }
        }
    }

    pub fn InitSimpleEx(
        ClientPortNumber: i32,
        WebPortNumber: i32,
        DependService: &[&Any],
    ) -> STARSRVGROUP {
        unsafe {
            if starrust_starcore_Load() != 0 {
                println!("{}","please download and install cle from http://www.srplab.com/en/files/products.html");
                panic!("starcore init fail");
            }

            starrust_starcore_Init(
                VS_TRUE,
                VS_FALSE,
                VS_FALSE,
                VS_TRUE,
                CString::new("").unwrap().as_ptr() as *const c_char,
                0,
                CString::new("").unwrap().as_ptr() as *const c_char,
                ClientPortNumber,
            );
            let BasicSRPInterface =
                Star_SRPControl_QueryBasicInterface(StarRust_SRPControlInterface, 0);

            for num in 0..DependService.len() {
                let DependServiceName = SRPRustGetStr(DependService[num], false);
                if vs_string_strlen(DependServiceName) != 0
                    && Star_SRPBasic_ImportService(BasicSRPInterface, DependServiceName, VS_TRUE)
                        == VS_FALSE
                {
                    Star_SRPBasic_Release(BasicSRPInterface);
                    panic!(format!{"import depend service {:?} fail", DependService[num]});
                }
                STARRUST_SAFERELEASESTR(DependService[num], DependServiceName);
            }

            if WebPortNumber != 0 {
                Star_SRPBasic_SetWebServerPort(
                    BasicSRPInterface,
                    CString::new("").unwrap().as_ptr() as *const c_char,
                    WebPortNumber as u16,
                    100,
                    2048,
                );
            }

            if WEAKTABLEMUTEXINITFLAG == false {
                WEAKTABLEMUTEXINITFLAG = true;
                WEAKTABLEMUTEX = starrust_mutex_init();
            }
            return RustSRPGetSrvGroup(0, BasicSRPInterface);
            //return ToStarSrvGroup(0, BasicSRPInterface);
        }
    }

    pub fn InitCore(
        ServerFlag: bool,
        ShowMenuFlag: bool,
        ShowOutWndFlag: bool,
        SRPPrintFlag: bool,
        DebugInterface: &Any,
        DebugPortNumber: i32,
        ClientInterface: &Any,
        ClientPortNumber: i32,
    ) -> i32 {
        unsafe {
            if starrust_starcore_Load() != 0 {
                println!("{}","please download and install cle from http://www.srplab.com/en/files/products.html");
                panic!("starcore init fail");
            }

            let LocalDebugInterface = SRPRustGetStr(DebugInterface, false);
            let LocalClientInterface = SRPRustGetStr(ClientInterface, false);

            starrust_starcore_Init(
                TOVS_BOOL(ServerFlag),
                TOVS_BOOL(ShowMenuFlag),
                TOVS_BOOL(ShowOutWndFlag),
                TOVS_BOOL(SRPPrintFlag),
                LocalDebugInterface as *const c_char,
                DebugPortNumber,
                LocalClientInterface as *const c_char,
                ClientPortNumber,
            );
            STARRUST_SAFERELEASESTR(DebugInterface, LocalDebugInterface);
            STARRUST_SAFERELEASESTR(ClientInterface, LocalClientInterface);

            if WEAKTABLEMUTEXINITFLAG == false {
                WEAKTABLEMUTEXINITFLAG = true;
                WEAKTABLEMUTEX = starrust_mutex_init();
            }

            if StarRust_ModuleInitFlag == VS_TRUE {
                return 0;
            } else {
                panic!("starcore init fail");
            }
        }
    }

    pub fn ModuleExit() {
        unsafe {
            let BasicSRPInterface: *const c_void;

            if StarRust_ModuleInitFlag == VS_FALSE {
                return;
            }

            BasicSRPInterface =
                Star_SRPControl_QueryBasicInterface(StarRust_SRPControlInterface, 0);
            if Star_SRPBasic_IsRootService(BasicSRPInterface) == VS_TRUE {
                Star_SRPBasic_ClearService(BasicSRPInterface);
                Star_SRPBasic_Release(BasicSRPInterface);
                RustSRPClearSrvGroup(true);

                if G_MSGCALLBACK as *const c_void != Default_MsgCallBack as *const c_void {
                    starrust_UnRegisterCallBackInfo(starrust_MsgCallBack as *mut c_void, 0);
                    G_MSGCALLBACK = Default_MsgCallBack;
                }
                if G_DISPATCHREQUESTCALLBACK != Default_DispatchRequestCallBack {
                    Star_SRPControl_UnRegDispatchRequest(
                        StarRust_SRPControlInterface,
                        starrust_SRPDispatchRequestCallBack as *mut c_void,
                        0,
                    );
                    G_DISPATCHREQUESTCALLBACK = Default_DispatchRequestCallBack;
                }

                starrust_starcore_unLoad();
            } else {
                //if dyna service, only term
                Star_SRPBasic_Release(BasicSRPInterface);
                starrust_Term();
            }
        }

        return;
    }

    pub fn ModuleClear() {
        unsafe {
            let BasicSRPInterface: *const c_void;

            if StarRust_ModuleInitFlag == VS_FALSE {
                return;
            }

            BasicSRPInterface =
                Star_SRPControl_QueryBasicInterface(StarRust_SRPControlInterface, 0);
            if Star_SRPBasic_IsRootService(BasicSRPInterface) == VS_TRUE {
                Star_SRPBasic_ClearService(BasicSRPInterface);
                Star_SRPBasic_Release(BasicSRPInterface);
                RustSRPClearSrvGroup(true);

                if G_MSGCALLBACK as *const c_void != Default_MsgCallBack as *const c_void {
                    starrust_UnRegisterCallBackInfo(starrust_MsgCallBack as *mut c_void, 0);
                    G_MSGCALLBACK = Default_MsgCallBack;
                }
                if G_DISPATCHREQUESTCALLBACK != Default_DispatchRequestCallBack {
                    Star_SRPControl_UnRegDispatchRequest(
                        StarRust_SRPControlInterface,
                        starrust_SRPDispatchRequestCallBack as *mut c_void,
                        0,
                    );
                    G_DISPATCHREQUESTCALLBACK = Default_DispatchRequestCallBack;
                }

                if StarRust_SRPControlInterface != 0 as *mut c_void {
                    Star_SRPControl_ClearScriptObject(
                        StarRust_SRPControlInterface,
                        starrust_FreeScriptObject as *mut c_void,
                        0,
                    );
                }

                if StarRust_SRPControlInterface != 0 as *mut c_void {
                    loop {
                        if Star_SRPControl_SRPDispatch(StarRust_SRPControlInterface, VS_FALSE)
                            == VS_FALSE
                        {
                            break;
                        }
                    }
                }
            } else {
                //if dyna service, only term
                Star_SRPBasic_Release(BasicSRPInterface);
                starrust_Term();
            }
        }
        return;
    }

    pub fn RegMsgCallBack_P(
        CallBackProc: fn(ServiceGroupID: u32, uMsg: u32, wParam: &Any, lParam: &Any)
            -> (bool, Box<Any>),
    ) {
        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return;
            }
            if CallBackProc as *const c_void == Default_MsgCallBack as *const c_void {
                if G_MSGCALLBACK as *const c_void != Default_MsgCallBack as *const c_void {
                    starrust_UnRegisterCallBackInfo(starrust_MsgCallBack as *mut c_void, 0);
                    G_MSGCALLBACK = Default_MsgCallBack;
                }
                return;
            }
            if G_MSGCALLBACK as *const c_void == Default_MsgCallBack as *const c_void {
                starrust_RegisterCallBackInfo(starrust_MsgCallBack as *mut c_void, 0);
            }
            G_MSGCALLBACK = CallBackProc;
        }
        return;
    }

    pub fn RegDispatchRequest_P(CallBackProc: fn()) {
        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return;
            }
            if CallBackProc == Default_DispatchRequestCallBack {
                if G_DISPATCHREQUESTCALLBACK != Default_DispatchRequestCallBack {
                    Star_SRPControl_UnRegDispatchRequest(
                        StarRust_SRPControlInterface,
                        starrust_SRPDispatchRequestCallBack as *mut c_void,
                        0,
                    );
                }
                G_DISPATCHREQUESTCALLBACK = Default_DispatchRequestCallBack;
                return;
            }
            if G_DISPATCHREQUESTCALLBACK == Default_DispatchRequestCallBack {
                Star_SRPControl_RegDispatchRequest(
                    StarRust_SRPControlInterface,
                    starrust_SRPDispatchRequestCallBack as *mut c_void,
                    0,
                );
            }
            G_DISPATCHREQUESTCALLBACK = CallBackProc;
        }
        return;
    }

    pub fn SRPDispatch(WaitFlag: bool) -> bool {
        let ResultVal: VS_BOOL;
        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return false;
            }
            if WaitFlag == true {
                ResultVal = Star_SRPControl_SRPDispatch(StarRust_SRPControlInterface, VS_TRUE);
            } else {
                ResultVal = Star_SRPControl_SRPDispatch(StarRust_SRPControlInterface, VS_FALSE);
            }
            return FROMVS_BOOL(ResultVal);
        }
    }

    pub fn SRPLock() {
        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return;
            }
            Star_SRPControl_SRPLock(StarRust_SRPControlInterface);
            return;
        }
    }

    pub fn SRPUnLock() {
        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return;
            }
            Star_SRPControl_SRPUnLock(StarRust_SRPControlInterface);
            return;
        }
    }

    pub fn SetRegisterCode(In_FileName: &Any, Single: bool) -> bool {
        let FileName: *mut c_char;
        let ResultVal: VS_BOOL;

        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return false;
            }
            FileName = SRPRustGetStr(In_FileName, false);
            ResultVal = Star_SRPControl_SetRegisterCode(
                StarRust_SRPControlInterface,
                FileName,
                TOVS_BOOL(Single),
            );
            STARRUST_SAFERELEASESTR(In_FileName, FileName);
            return FROMVS_BOOL(ResultVal);
        }
    }

    pub fn IsRegistered() -> bool {
        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return false;
            }
            return FROMVS_BOOL(Star_SRPControl_IsRegistered(StarRust_SRPControlInterface));
        }
    }

    pub fn GetLocale() -> String {
        let Lang: *mut c_char;

        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return "".to_owned();
            }
            Lang = Star_SRPControl_GetLocale(StarRust_SRPControlInterface);
            return CStr::from_ptr(Lang).to_str().unwrap().to_owned();
        }
    }

    pub fn Version() -> [i32; 3] {
        let mut IntArray: [i32; 3] = [0, 0, 0];

        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return IntArray;
            }
            let BasicSRPInterface =
                Star_SRPControl_QueryBasicInterface(StarRust_SRPControlInterface, 0);
            let mut MainVersion: u8 = 0;
            let mut SubVersion: u8 = 0;
            let mut BuildVersion: u16 = 0;
            Star_SRPBasic_GetVersion(
                BasicSRPInterface,
                &mut MainVersion,
                &mut SubVersion,
                &mut BuildVersion,
            );
            IntArray[0] = MainVersion as i32;
            IntArray[1] = SubVersion as i32;
            IntArray[2] = BuildVersion as i32;
            Star_SRPBasic_Release(BasicSRPInterface);
            return IntArray;
        }
    }

    pub fn GetScriptIndex(In_ScriptInterface: &Any) -> i32 {
        let ScriptInterface: *mut c_char;
        let Index: i32;

        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return -1;
            }
            ScriptInterface = SRPRustGetStr(In_ScriptInterface, false);
            Index = Star_SRPControl_GetScriptInterfaceIndex(
                StarRust_SRPControlInterface,
                ScriptInterface,
            );
            STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);
            return Index;
        }
    }

    pub fn SetScript(In_ScriptInterface: &Any, In_Module: &Any, In_Para: &Any) -> bool {
        let ScriptInterface: *mut c_char;
        let Module: *mut c_char;
        let Para: *mut c_char;
        let ResultVal: VS_BOOL;

        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return false;
            }
            ScriptInterface = SRPRustGetStr(In_ScriptInterface, false);
            Module = SRPRustGetStr(In_Module, false);
            Para = SRPRustGetStr(In_Para, false);
            ResultVal = Star_SRPControl_SetScriptInterface(
                StarRust_SRPControlInterface,
                ScriptInterface,
                Module,
                Para,
            );
            STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);
            STARRUST_SAFERELEASESTR(In_Module, Module);
            STARRUST_SAFERELEASESTR(In_Para, Para);
            return FROMVS_BOOL(ResultVal);
        }
    }

    pub fn DetachCurrentThread() {
        unsafe {
            if StarRust_SRPControlInterface == 0 as *mut c_void {
                return;
            }
            Star_SRPControl_DetachCurrentThread(StarRust_SRPControlInterface);
            return;
        }
    }

    pub fn CoreHandle() -> i64 {
        unsafe {
            return starrust_CoreHandle() as i64;
        }
    }

    /*--i32 or String */
    pub fn GetSrvGroup(In_ServiceName_GroupID: &Any) -> STARSRVGROUP {
        unsafe {
            if let Some(ServiceName_GroupID) = In_ServiceName_GroupID.downcast_ref::<i32>() {
                let BasicSRPInterface = Star_SRPControl_QueryBasicInterface(
                    StarRust_SRPControlInterface,
                    *ServiceName_GroupID as u32,
                );
                if BasicSRPInterface == 0 as *mut c_void {
                    return STARRC!(None);
                }
                let RetObject = RustSRPGetSrvGroup(0, BasicSRPInterface);
                Star_SRPBasic_Release(BasicSRPInterface);
                return RetObject;
            } else if let Some(ServiceName_GroupID) =
                In_ServiceName_GroupID.downcast_ref::<String>()
            {
                let mut ActiveServiceName: *mut c_char;
                let mut ServiceID: VS_UUID = VS_UUID::new();

                let ServiceName = CString::new(ServiceName_GroupID.clone()).unwrap().as_ptr();

                let mut ServiceGroupID =
                    Star_SRPControl_QueryFirstServiceGroup(StarRust_SRPControlInterface);
                let mut BasicSRPInterface: *const c_void = 0 as *const c_void;
                loop {
                    if ServiceGroupID == VS_INVALID_SERVICEGROUPID {
                        break;
                    }
                    BasicSRPInterface = Star_SRPControl_QueryBasicInterface(
                        StarRust_SRPControlInterface,
                        ServiceGroupID,
                    );
                    if BasicSRPInterface == 0 as *mut c_void {
                        return STARRC!(None);
                    }
                    ActiveServiceName =
                        Star_SRPBasic_QueryActiveService(BasicSRPInterface, &mut ServiceID);
                    if ActiveServiceName != 0 as *mut c_char
                        && vs_string_strcmp(ActiveServiceName, ServiceName) == 0
                    {
                        break;
                    }
                    Star_SRPBasic_Release(BasicSRPInterface);
                    ServiceGroupID =
                        Star_SRPControl_QueryNextServiceGroup(StarRust_SRPControlInterface);
                }
                if ServiceGroupID == VS_INVALID_SERVICEGROUPID {
                    Star_SRPBasic_Release(BasicSRPInterface);
                    return STARRC!(None);
                }
                let RetObject = RustSRPGetSrvGroup(ServiceGroupID, BasicSRPInterface);
                Star_SRPBasic_Release(BasicSRPInterface);
                return RetObject;
            } else {
                let BasicSRPInterface =
                    Star_SRPControl_QueryBasicInterface(StarRust_SRPControlInterface, 0);
                if BasicSRPInterface == 0 as *mut c_void {
                    return STARRC!(None);
                }
                let RetObject = RustSRPGetSrvGroup(0, BasicSRPInterface);
                Star_SRPBasic_Release(BasicSRPInterface);
                return RetObject;
            }
        }
    }

    pub fn RegScriptTermCallBack_P(CallBackProc: fn()) {
        unsafe{
	        G_SCRIPTTERMCALLBACK = CallBackProc;
        }
    }

    pub fn RegScriptInitCallBack_P(CallBackProc: fn(SrvGroup: &STARSRVGROUP, Service: &STARSERVICE)) {
        unsafe{
	        G_SCRIPTINITCALLBACK = CallBackProc;
        }
    }

    /*----------------------------------------------------------------------------*/

    pub fn print(val: String) {
        unsafe{
    	    if StarRust_ModuleLoadFromStarcore == VS_FALSE {
	    	    print!("{}",val);
	        } else {
		        let cstr = SRPRustGetStr(&val, true);
        		Star_SRPControl_SRPLock(StarRust_SRPControlInterface);
	        	starrust_SRPControl_ProcessError(StarRust_SRPControlInterface, VSFAULT_INDICATION, CString::new("rust").unwrap().as_ptr(), 0, cstr);
		        Star_SRPControl_SRPUnLock(StarRust_SRPControlInterface);
        		Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
	        	return;
    	    }
        }
    }  

    pub fn printw(val: String) {
        unsafe{
    	    if StarRust_ModuleLoadFromStarcore == VS_FALSE {
	    	    print!("{}",val);
	        } else {
		        let cstr = SRPRustGetStr(&val, true);
        		Star_SRPControl_SRPLock(StarRust_SRPControlInterface);
	        	starrust_SRPControl_ProcessError(StarRust_SRPControlInterface, VSFAULT_WARNING, CString::new("rust").unwrap().as_ptr(), 0, cstr);
		        Star_SRPControl_SRPUnLock(StarRust_SRPControlInterface);
    		    Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
    	    	return;
        	}
        }
    }      

    pub fn printe(val: String) {
        unsafe{
    	    if StarRust_ModuleLoadFromStarcore == VS_FALSE {
	    	    print!("{}",val);
	        } else {
		        let cstr = SRPRustGetStr(&val, true);
        		Star_SRPControl_SRPLock(StarRust_SRPControlInterface);
	        	starrust_SRPControl_ProcessError(StarRust_SRPControlInterface, VSFAULT_NORMALERROR, CString::new("rust").unwrap().as_ptr(), 0, cstr);
		        Star_SRPControl_SRPUnLock(StarRust_SRPControlInterface);
    		    Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
    	    	return;
        	}
        }
    }   

    pub fn println(val: String) {
        unsafe{
    	    if StarRust_ModuleLoadFromStarcore == VS_FALSE {
	    	    println!("{}",val);
	        } else {
		        let cstr = SRPRustGetStr(&val, true);
        		Star_SRPControl_SRPLock(StarRust_SRPControlInterface);
	        	starrust_SRPControl_ProcessError(StarRust_SRPControlInterface, VSFAULT_INDICATION, CString::new("rust").unwrap().as_ptr(), 0, cstr);
		        Star_SRPControl_SRPUnLock(StarRust_SRPControlInterface);
        		Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
	        	return;
    	    }
        }
    }  

    pub fn printlnw(val: String) {
        unsafe{
    	    if StarRust_ModuleLoadFromStarcore == VS_FALSE {
	    	    println!("{}",val);
	        } else {
		        let cstr = SRPRustGetStr(&val, true);
        		Star_SRPControl_SRPLock(StarRust_SRPControlInterface);
	        	starrust_SRPControl_ProcessError(StarRust_SRPControlInterface, VSFAULT_WARNING, CString::new("rust").unwrap().as_ptr(), 0, cstr);
		        Star_SRPControl_SRPUnLock(StarRust_SRPControlInterface);
    		    Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
    	    	return;
        	}
        }
    }      

    pub fn printlne(val: String) {
        unsafe{
    	    if StarRust_ModuleLoadFromStarcore == VS_FALSE {
	    	    println!("{}",val);
	        } else {
		        let cstr = SRPRustGetStr(&val, true);
        		Star_SRPControl_SRPLock(StarRust_SRPControlInterface);
	        	starrust_SRPControl_ProcessError(StarRust_SRPControlInterface, VSFAULT_NORMALERROR, CString::new("rust").unwrap().as_ptr(), 0, cstr);
		        Star_SRPControl_SRPUnLock(StarRust_SRPControlInterface);
    		    Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
    	    	return;
        	}
        }
    }        


    pub fn get_env(Name: &Any) -> String{
        let mut Buf: [c_char;512] = unsafe{mem::zeroed()};
        let cstr = SRPRustGetStr(Name, false);
        if unsafe{vs_get_env(cstr,Buf.as_mut_ptr(),512)} == VS_TRUE {
            STARRUST_SAFERELEASESTR(Name,cstr);
            return SRPRustSetStr(Buf.as_mut_ptr(), true);
        }else{
            STARRUST_SAFERELEASESTR(Name,cstr);
            return "".to_owned();
        }
    }
    pub fn set_env(Name: &Any,Buf: &Any) -> bool{
        let cstr = SRPRustGetStr(Name, false);
        let cstr1 = SRPRustGetStr(Buf, true);
        let ret = unsafe{vs_set_env(cstr,cstr1)};
        STARRUST_SAFERELEASESTR(Name,cstr);
        unsafe{Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr1 as *mut c_void);}
        return FROMVS_BOOL(ret);
    }    

    pub fn ToString(val:&Any) -> String{
        if let Some(ObjData) = val.downcast_ref::<String>() {
            return ObjData.clone();
        }else if let Some(ObjData) = val.downcast_ref::<&str>() {
            return ObjData.to_string();
        }else{
            return "".to_owned();
        }
    }

    pub fn ToBool(val:&Any) -> bool{
        if let Some(ObjData) = val.downcast_ref::<bool>() {
            return *ObjData;
        }else{
            return false;
        }
    }   

    pub fn ToInt(Val:&Any) -> i32{
        return SRPRustToInt(Val, true) as i32;
    }     

    pub fn ToInt64(Val:&Any) -> i64{
        return SRPRustToInt64(Val, true) as i64;
    }         

    pub fn ToDouble(Val:&Any) -> f64{
        return SRPRustToFloat(Val) as f64;
    }  

}

/*----------------------------------------------------------------------------*/
/* StarSrvGroup */
/*  note: should not hold the borrow
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCRef_STARSRVGROUP!(self) {    
                StarSrvGroupBody = ObjData.ObjData.clone();
            }else{
                return;
            }  
----------------------------------------------------------------------------*/
pub trait STARSRVGROUP_TRAIT {
    fn IsValid(&self) -> bool;
    fn ToString(&self) -> String;

    fn CreateService(&self,In_ServicePath: &Any, In_ServiceName: &Any, In_RootPass: &Any, FrameInterval: i32, NetPkgSize: i32, UploadPkgSize: i32, DownloadPkgSize: i32, DataUpPkgSize: i32, DataDownPkgSize: i32, In_ServiceID: &Any) -> STARSERVICE;
    fn GetService(&self,username: &Any, userpassword: &Any) -> STARSERVICE;
    fn ClearService(&self);

    fn NewParaPkg(&self,args: &[&Any]) -> STARPARAPKG;
    fn NewParaDict(&self,args: &[&Any]) -> STARPARAPKG;
    fn NewBinBuf(&self) -> STARBINBUF;
    fn NewSXml(&self) -> STARSXML;
    fn IsObject(&self,Arg:&Any) -> bool;
    fn IsParaPkg(&self,Arg:&Any) -> bool;
    fn IsBinBuf(&self,Arg:&Any) -> bool;
    fn IsSXml(&self,Arg:&Any) -> bool; 

    fn GetServicePath(&self) -> String;
    fn SetServicePath(&self,Args: &Any);
    fn ServicePathIsSet(&self) -> bool;
    fn GetCurrentPath(&self) -> String;   

    fn ImportService(&self,In_ServiceName: &Any, LoadRunModule: bool) -> bool;
    fn ClearServiceEx(&self);
    fn RunScript(&self,In_ScriptInterface: &Any, In_ScriptBuf: &Any, In_Name: &Any) -> bool;
    fn RunScriptEx(&self,In_ScriptInterface: &Any, BinBuf: &STARBINBUF, In_Name: &Any) -> bool;
    fn DoFile(&self,In_ScriptInterface: &Any, In_FileName: &Any) -> bool;
    fn DoFileEx(&self,In_ScriptInterface: &Any, In_FileName: &Any, In_ModuleName: &Any) -> bool;
    fn SetClientPort(&self,LInterface: &Any, Portnumber: isize) -> bool;
    fn SetTelnetPort(&self,Portnumber: isize) -> bool;
    fn SetOutputPort(&self,Host: &Any, Portnumber: isize) -> bool;
    fn SetWebServerPort(&self,In_Host: &Any, Portnumber: isize, ConnectionNumber: isize, PostSize: isize) -> bool;

    fn InitRaw(&self,ScriptInterface: &Any, Service: &STARSERVICE) -> bool;
    fn LoadRawModule(&self,ScriptInterface: &Any, ModuleName: &Any, FileOrString: &Any, IsString: bool) -> bool;
    fn GetLastError(&self) -> isize;
    fn GetLastErrorInfo(&self) -> String;
    fn SUnLockGC(&self);
    fn GetCorePath(&self) -> String;
    fn GetUserPath(&self) -> String;
    fn GetLocalIP(&self) -> String;
    fn GetLocalIPEx(&self) -> Vec<String>;
    fn GetObjectNum(&self) -> isize;  

     fn ActiveScriptInterface(&self,ScriptInterface: &Any) -> (bool,bool);
     fn PreCompile(&self,ScriptInterface: &Any, ScriptBuf: &Any) -> (bool,String);

}

impl STARSRVGROUP_TRAIT for STARSRVGROUP {
    fn IsValid(&self) -> bool {
        if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {
            return ObjData.IsValid;
        } else {
            return false;
        }
    }
    fn ToString(&self) -> String {
        let st: String;

        if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {
            st = format!("{}", ObjData);
        } else {
            st = "srvgroup object is invalid".to_owned();
        }
        return st;
    }
    fn CreateService(&self,In_ServicePath: &Any, In_ServiceName: &Any, In_RootPass: &Any, FrameInterval: i32, NetPkgSize: i32, UploadPkgSize: i32, DownloadPkgSize: i32, DataUpPkgSize: i32, DataDownPkgSize: i32, In_ServiceID: &Any) -> STARSERVICE
    {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(self); 
            if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {                
    	        let ServicePath = SRPRustGetStr(In_ServicePath, true);
	            let ServiceName = SRPRustGetStr(In_ServiceName, false);
	            let RootPass = SRPRustGetStr(In_RootPass, false);
	            let ServiceID = SRPRustGetStr(In_ServiceID, false);
                let mut ObjectID : VS_UUID = VS_UUID::new();

    	        if ServiceID != 0 as *mut c_char {
	    	        Star_SRPBasic_StringToUuid(ObjData.ObjData.BasicSRPInterface, ServiceID as *const i8, &mut ObjectID);
            	} else {
            		starrust_inituuid(&mut ObjectID);
	            }
	            let ResultVar = Star_SRPBasic_CreateService(ObjData.ObjData.BasicSRPInterface, ServicePath, ServiceName, &mut ObjectID, RootPass, FrameInterval, NetPkgSize, UploadPkgSize, DownloadPkgSize, DataUpPkgSize, DataDownPkgSize);
	            if ResultVar == VS_TRUE {
            		Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, ServicePath as *mut c_void);
		            let SRPInterface = Star_SRPBasic_GetSRPInterface(ObjData.ObjData.BasicSRPInterface, ServiceName,CString::new("root").unwrap().as_ptr(), RootPass);
		            STARRUST_SAFERELEASESTR(In_ServiceName, ServiceName);
		            STARRUST_SAFERELEASESTR(In_RootPass, RootPass);
		            STARRUST_SAFERELEASESTR(In_ServiceID, ServiceID);
		            return ToStarService(&mut ObjData.ObjData, SRPInterface);
	            }
           		Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, ServicePath as *mut c_void);
	            STARRUST_SAFERELEASESTR(In_ServiceName, ServiceName);
	            STARRUST_SAFERELEASESTR(In_RootPass, RootPass);
	            STARRUST_SAFERELEASESTR(In_ServiceID, ServiceID);
	            return STARRC!(None);
            }else{
                return STARRC!(None);
            }
        }
    } 
    fn GetService(&self,username: &Any, userpassword: &Any) -> STARSERVICE
    {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(self); 
            if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {                
    	        let CharTemp1 = SRPRustGetStr(username, false);
	            let CharTemp2 = SRPRustGetStr(userpassword, false);

                let mut ServiceID :VS_UUID = VS_UUID::new();

	            if Star_SRPBasic_QueryActiveService(ObjData.ObjData.BasicSRPInterface, &mut ServiceID) == 0 as *mut c_char {
		            STARRUST_SAFERELEASESTR(username, CharTemp1);
		            STARRUST_SAFERELEASESTR(userpassword, CharTemp2);
		            return STARRC!(None);
	            }
            	let ObjectTemp = RustSRPQueryServiceByServiceID(&mut ObjData.ObjData, &ServiceID);
	            if IsStarServiceClassObject(&ObjectTemp) {
		            STARRUST_SAFERELEASESTR(username, CharTemp1);
		            STARRUST_SAFERELEASESTR(userpassword, CharTemp2);
		            return ObjectTemp
	            }
            	let SRPInterface = Star_SRPBasic_GetSRPInterfaceEx(ObjData.ObjData.BasicSRPInterface, &mut ServiceID, CharTemp1, CharTemp2);
            	STARRUST_SAFERELEASESTR(username, CharTemp1);
            	STARRUST_SAFERELEASESTR(userpassword, CharTemp2);
	            if SRPInterface == 0 as *mut c_void {
		            return STARRC!(None);
            	}
            	return ToStarService(&mut ObjData.ObjData, SRPInterface);   
            }else{
                return STARRC!(None);
            } 
        }
    }   
    fn ClearService(&self)
    {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return;
            }  
            Star_SRPBasic_ClearService(fbody.BasicSRPInterface);
            return;
        }
    }

    fn NewParaPkg(&self,args: &[&Any]) -> STARPARAPKG
    {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return STARRC!(None);
            }
            let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(fbody.BasicSRPInterface);
            if args.len() != 0 {
           		let SRPInterface = GetSRPServiceInterfaceEx(Star_SRPBasic_GetServiceGroupID(fbody.BasicSRPInterface), 0 as *mut VS_UUID);
           		if SRPInterface == 0 as *mut c_void {
		            starrust_SRPBasic_ProcessError(fbody.BasicSRPInterface, VSFAULT_WARNING, CString::new("rust").unwrap().as_ptr(), 0, CString::new("service must be created").unwrap().as_ptr());
           		} else {
		            if StarParaPkg_FromTuple_Sub(args, NewParaPkg, fbody.BasicSRPInterface, 0, SRPInterface) == false {
                        starrust_SRPBasic_ProcessError(fbody.BasicSRPInterface, VSFAULT_WARNING, CString::new("rust").unwrap().as_ptr(), 0, CString::new("parameter not supported for starparapkg").unwrap().as_ptr());
                    }
       			}
       		}
           	return ToStarParaPkg(NewParaPkg, Star_SRPBasic_GetServiceGroupID(fbody.BasicSRPInterface), true);
       }    
    }

    fn NewParaDict(&self,args: &[&Any]) -> STARPARAPKG
    {
        let Para = self.NewParaPkg(args);
        Para.AsDict(true);
        return Para;
    }    

    fn NewBinBuf(&self) -> STARBINBUF {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return STARRC!(None);
            }
	        return CreateStarBinBuf(fbody.BasicSRPInterface as *mut c_void);
        }
    }

    fn NewSXml(&self) -> STARSXML {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return STARRC!(None);
            }
	        return CreateStarSXml(fbody.BasicSRPInterface as *mut c_void);
        }
    }    

    fn IsObject(&self,Arg:&Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            return IsStarObjectClassObject(Arg);
        }
    }       

    fn IsParaPkg(&self,Arg:&Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            return IsStarParaPkgClassObject(Arg);
        }
    } 

    fn IsBinBuf(&self,Arg:&Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            return IsStarBinBufClassObject(Arg);
        }
    } 

    fn IsSXml(&self,Arg:&Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            return IsStarSXmlClassObject(Arg);
        }
    } 
    
    fn GetServicePath(&self) -> String {
        unsafe{
            let LocalBuf : [c_char;512] = mem::zeroed();
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return "".to_owned();
            }            
            Star_SRPBasic_GetDefaultPath(fbody.BasicSRPInterface, &LocalBuf[0] as *const c_char as *mut c_char,512);
            return SRPRustSetStr(&LocalBuf[0] as *const c_char as *mut c_char, true);
        }
    }

    fn SetServicePath(&self,Args: &Any) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return;
            }
            let CharTemp = SRPRustGetStr(Args, true);
            Star_SRPBasic_SetDefaultPath(fbody.BasicSRPInterface, CharTemp);
            Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, CharTemp as *mut c_void);
        }
    }

    fn ServicePathIsSet(&self) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }
	        return FROMVS_BOOL(Star_SRPBasic_DefaultPathIsSet(fbody.BasicSRPInterface));
        }
    }

    fn GetCurrentPath(&self) -> String {
        unsafe{        
            let LocalBuf : [c_char;512] = mem::zeroed();
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
	        vs_dir_getcwd(&LocalBuf[0] as *const c_char as *mut c_char, 512);
	        return SRPRustSetStr(&LocalBuf[0] as *const c_char as *mut c_char, true);            
        }
    }

    fn ImportService(&self,In_ServiceName: &Any, LoadRunModule: bool) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }
        	let ServiceName = SRPRustGetStr(In_ServiceName, true);
        	let ResultVal = Star_SRPBasic_ImportService(fbody.BasicSRPInterface, ServiceName, TOVS_BOOL(LoadRunModule));
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface,ServiceName as *mut c_void);
	        return FROMVS_BOOL(ResultVal);
        }
    }

    fn ClearServiceEx(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return;
            }
            Star_SRPBasic_ClearServiceEx(fbody.BasicSRPInterface);
        }
    }

    fn RunScript(&self,In_ScriptInterface: &Any, In_ScriptBuf: &Any, In_Name: &Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }
        	let ScriptInterface = SRPRustGetStr(In_ScriptInterface, false);
	        let ScriptBuf = SRPRustGetStr(In_ScriptBuf, true);
	        let ScriptName = SRPRustGetStr(In_Name, false);
	        let RetResult = Star_SRPBasic_DoBuffer(fbody.BasicSRPInterface, ScriptInterface, ScriptBuf as *const i8, vs_string_strlen(ScriptBuf) as i32, VS_FALSE, ScriptName);
        	STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, ScriptBuf as *mut c_void);
	        STARRUST_SAFERELEASESTR(In_Name, ScriptName);
	        return FROMVS_BOOL(RetResult);
        }
    }

    fn RunScriptEx(&self,In_ScriptInterface: &Any, BinBuf: &STARBINBUF, In_Name: &Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }        
        	let ScriptInterface = SRPRustGetStr(In_ScriptInterface, false);
	        let ScriptName = SRPRustGetStr(In_Name, false);
	        let SRPBinBufInterface = RustObjectToBinBuf(BinBuf);
	        if SRPBinBufInterface == 0 as *mut c_void || Star_SRPBinBuf_GetBufPtr(SRPBinBufInterface, 0) == 0 as *mut i8 {
        		STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);
		        STARRUST_SAFERELEASESTR(In_Name, ScriptName);
		        return false;
	        }
	        let RetResult = Star_SRPBasic_DoBuffer(fbody.BasicSRPInterface, ScriptInterface, Star_SRPBinBuf_GetBufPtr(SRPBinBufInterface, 0), Star_SRPBinBuf_GetOffset(SRPBinBufInterface) as i32, VS_FALSE, ScriptName);
        	STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);
	        STARRUST_SAFERELEASESTR(In_Name, ScriptName);
	        return FROMVS_BOOL(RetResult);
        }
    }

    fn DoFile(&self,In_ScriptInterface: &Any, In_FileName: &Any) -> bool {
	    return self.DoFileEx(In_ScriptInterface, In_FileName, &"");
    }

    fn DoFileEx(&self,In_ScriptInterface: &Any, In_FileName: &Any, In_ModuleName: &Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }           
	        let ScriptInterface = SRPRustGetStr(In_ScriptInterface, false);
	        let FileName = SRPRustGetStr(In_FileName, true);
	        let ModuleName = SRPRustGetStr(In_ModuleName, false);
	        let ResultVal = Star_SRPBasic_DoFileEx(fbody.BasicSRPInterface, ScriptInterface, FileName,0 as *mut [*mut c_char;1], 0 as *mut c_char, VS_FALSE, ModuleName);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, FileName as *mut c_void);
	        STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);
	        STARRUST_SAFERELEASESTR(In_ModuleName, ModuleName);
	        return FROMVS_BOOL(ResultVal);
        }
    }

    fn SetClientPort(&self,LInterface: &Any, Portnumber: isize) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }            
	        let InterfaceValue = SRPRustGetStr(LInterface, false);
	        let RetResult = Star_SRPBasic_SetClientPort(fbody.BasicSRPInterface, InterfaceValue, Portnumber as u16);
	        STARRUST_SAFERELEASESTR(LInterface, InterfaceValue);
	        return FROMVS_BOOL(RetResult);
        }
    }

    fn SetTelnetPort(&self,Portnumber: isize) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }   
	        return FROMVS_BOOL(Star_SRPBasic_SetTelnetPort(fbody.BasicSRPInterface,Portnumber as u16));
        }
    }

    fn SetOutputPort(&self,Host: &Any, Portnumber: isize) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }         
	        let InterfaceValue = SRPRustGetStr(Host, false);
	        let RetResult = Star_SRPBasic_SetOutputPort(fbody.BasicSRPInterface, InterfaceValue, Portnumber as u16);
	        STARRUST_SAFERELEASESTR(Host, InterfaceValue);
	        return FROMVS_BOOL(RetResult);
        }
    }

    fn SetWebServerPort(&self,In_Host: &Any, Portnumber: isize, ConnectionNumber: isize, PostSize: isize) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            } 
        	let Host = SRPRustGetStr(In_Host, false);
        	let RetResult = Star_SRPBasic_SetWebServerPort(fbody.BasicSRPInterface, Host, Portnumber as u16, ConnectionNumber as i32, PostSize as u32);
	        STARRUST_SAFERELEASESTR(In_Host, Host);
	        return FROMVS_BOOL(RetResult);
        }    
    }

    fn InitRaw(&self,ScriptInterface: &Any, Service: &STARSERVICE) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }         
        	let SRPInterface = RustObjectToSRPServiceInterface(Service);
	        if SRPInterface == 0 as *mut c_void {
		        return false;
	        }
	        let In_ScriptInterface = SRPRustGetStr(ScriptInterface, false);
            
	        let ResultVal = Star_SRPBasic_InitRaw(fbody.BasicSRPInterface, In_ScriptInterface, SRPInterface);
	        STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
	        return FROMVS_BOOL(ResultVal);
        }
    }

    fn LoadRawModule(&self,ScriptInterface: &Any, ModuleName: &Any, FileOrString: &Any, IsString: bool) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return false;
            }           
        	let In_ScriptInterface = SRPRustGetStr(ScriptInterface, false);
	        let In_FileOrString = SRPRustGetStr(FileOrString, true);
	        let In_ModuleName = SRPRustGetStr(ModuleName, true);
	        let ResultVal = Star_SRPBasic_LoadRawModule(fbody.BasicSRPInterface, In_ScriptInterface, In_ModuleName, In_FileOrString, TOVS_BOOL(IsString), 0 as *mut [*mut c_char;1]);
        	STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, In_FileOrString as *mut c_void);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, In_ModuleName as *mut c_void);
	        return FROMVS_BOOL(ResultVal);
        }
    }

    fn GetLastError(&self) -> isize {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return 0;
            }   
        	return Star_SRPBasic_GetLastError(fbody.BasicSRPInterface) as isize;
        }
    }

    fn GetLastErrorInfo(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return "".to_owned();
            }   
            let mut LineIndex : u32 = 0;
            let mut SourceName : Vec<*mut c_char> = Vec::new();
            SourceName.push(0 as *mut c_char);
	        let TextBuf = Star_SRPBasic_GetLastErrorInfo(fbody.BasicSRPInterface, &mut LineIndex,SourceName.as_mut_ptr() as usize as *mut [*mut c_char;1]);
	        return format!("[{}:{}]{}", SRPRustSetStr(SourceName[0],false), LineIndex, SRPRustSetStr(TextBuf, true));
        }
    }

    fn SUnLockGC(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return;
            } 
	        Star_SRPBasic_SUnLockGC(fbody.BasicSRPInterface);
        }
    }

    fn GetCorePath(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return "".to_owned();
            }         
        	return SRPRustSetStr(Star_SRPBasic_GetCorePath(fbody.BasicSRPInterface), true);
        }
    }

    fn GetUserPath(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return "".to_owned();
            }         
        	return SRPRustSetStr(Star_SRPBasic_GetUserPath(fbody.BasicSRPInterface), true);
        }
    }

    fn GetLocalIP(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "127.0.0.1".to_owned();
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return "127.0.0.1".to_owned();
            } 
        	return SRPRustSetStr(Star_SRPBasic_GetLocalIP(fbody.BasicSRPInterface), true);
        }
    }

    fn GetLocalIPEx(&self) -> Vec<String> {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return Vec::new();
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return Vec::new();
            } 
	        let mut SockAddr : Vec<VSSOCKADDR_IN> = Vec::new();
            for i in 0..64{
                SockAddr.push(VSSOCKADDR_IN::new());
            }
	        let mut RetObject : Vec<String> = Vec::new();
	        let Number = Star_SRPBasic_GetLocalIPEx(fbody.BasicSRPInterface,SockAddr.as_mut_ptr() as* mut VSSOCKADDR_IN, 64);
        	if Number == 0 {
		        return Vec::new();
	        }
	        for i in 0..Number as usize {
		        RetObject.push(SRPRustSetStr(starrust_IPToString((SockAddr.as_mut_ptr() as usize + i * size_of::<VSSOCKADDR_IN>()) as *const VSSOCKADDR_IN),false));
	        }
	        return RetObject;
        }
    }

    fn GetObjectNum(&self) -> isize {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return 0;
            }         
        	return Star_SRPBasic_GetObjectNum(fbody.BasicSRPInterface) as isize;
        }    
    }

    fn ActiveScriptInterface(&self,ScriptInterface: &Any) ->(bool,bool) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,false);
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return (false,false);
            } 

	        let In_ScriptInterface = SRPRustGetStr(ScriptInterface, false);
	        if fbody.BasicSRPInterface == 0 as *mut c_void {
                return (false,false);
        	}
	        if In_ScriptInterface == 0 as *mut c_char || vs_string_strlen(In_ScriptInterface) == 0 {
                return (false,false);
	        }

            let mut OnLineScriptFlag : VS_BOOL = VS_FALSE;

        	let Control = Star_SRPBasic_GetSRPControlInterface(fbody.BasicSRPInterface);
	        let ResultVal = Star_SRPControl_ActiveScriptInterface(Control, In_ScriptInterface,&mut OnLineScriptFlag, 0 as *mut c_void);
        	Star_SRPControl_Release(Control);
	        return (FROMVS_BOOL(ResultVal),FROMVS_BOOL(OnLineScriptFlag));
        }
    }

    fn PreCompile(&self,ScriptInterface: &Any, ScriptBuf: &Any) -> (bool,String) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,"call\"_PreCompile\",input para error".to_owned());
	        }
            let fbody : StarSrvGroupBody;
            if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return (false,"call\"_PreCompile\",input para error".to_owned());
            } 
	        let In_ScriptInterface = SRPRustGetStr(ScriptInterface, false);
    	    let In_ScriptBuf = SRPRustGetStr(ScriptBuf, false);
	        if fbody.BasicSRPInterface == 0 as *mut c_void {
        		STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
        		STARRUST_SAFERELEASESTR(ScriptBuf, In_ScriptBuf);
		        return (false,"call\"_PreCompile\",input para error".to_owned());
	        }
	        if In_ScriptInterface == 0 as *mut c_char || vs_string_strlen(In_ScriptInterface) == 0 || In_ScriptBuf == 0 as *mut c_char {
        		STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
		        STARRUST_SAFERELEASESTR(ScriptBuf, In_ScriptBuf);
		        return (false,"call\"_PreCompile\",input para error".to_owned());
	        }
            let mut ErrorInfo : Vec<*mut c_char> = Vec::new();
            ErrorInfo.push(0 as *mut c_char);            
	        let ResultVal = Star_SRPBasic_PreCompile(fbody.BasicSRPInterface, In_ScriptInterface, In_ScriptBuf as *const i8, vs_string_strlen(In_ScriptBuf) as i32, CString::new("").unwrap().as_ptr(),ErrorInfo.as_mut_ptr() as *mut [*mut c_char;1]);
        	if ErrorInfo[0] == 0 as *mut c_char {
		        STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
        		STARRUST_SAFERELEASESTR(ScriptBuf, In_ScriptBuf);
		        return (FROMVS_BOOL(ResultVal),"".to_owned());
        	} else {
        		STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
		        STARRUST_SAFERELEASESTR(ScriptBuf, In_ScriptBuf);
		        return (FROMVS_BOOL(ResultVal),SRPRustSetStr(ErrorInfo[0], false));
	        }
        }    
    }
}




/*----------------------------------------------------------------------------*/
/* StarService */
/*----------------------------------------------------------------------------*/
pub trait STARSERVICE_TRAIT {
    fn IsValid(&self) -> bool;
    fn ToString(&self) -> String;

    fn Get(&self,Name: &Any) -> STARRESULT;
    fn GetObject(&self,ObjectName: &Any) -> STAROBJECT;
    fn GetObjectEx(&self,ObjectID: &Any) -> STAROBJECT;
    fn New(&self,args: &[&Any]) -> STAROBJECT;
    fn RunScript(&self,In_ScriptInterface: &Any, In_ScriptBuf: &Any, In_Name: &Any, In_WorkDirectory: &Any) -> (bool,String);
    fn RunScriptEx(&self,In_ScriptInterface: &Any, BinBuf: &STARBINBUF, In_Name: &Any, In_WorkDirectory: &Any) -> (bool,String);
    fn DoFile(&self,In_ScriptInterface: &Any, In_FileName: &Any, In_WorkDirectory: &Any) -> (bool,String);
    fn DoFileEx(&self,In_ScriptInterface: &Any, In_FileName: &Any, In_WorkDirectory: &Any, In_ModuleName: &Any)-> (bool,String);
    fn IsServiceRegistered(&self) -> bool;
    fn CheckPassword(&self,Flag: bool);

    fn LoadRawModule(&self,ScriptInterface: &Any, ModuleName: &Any, FileOrString: &Any, IsString: bool) -> bool;
    fn NewRawProxy(&self,ScriptInterface: &Any, AttachObject: &STAROBJECT, AttachFunction: &Any, ProyInfo: &Any, ProxyType: isize) -> STAROBJECT;
    fn ImportRawContext(&self,ScriptInterface: &Any, ContextName: &Any, IsClass: bool, ContextInfo: &Any) -> STAROBJECT;
    fn GetLastError(&self) -> isize;
    fn GetLastErrorInfo(&self) -> String;
}

impl STARSERVICE_TRAIT for STARSERVICE {
    fn IsValid(&self) -> bool {
        if let Some(ObjData) = STARGETRCREF_STARSERVICE!(self) {
            return ObjData.IsValid;
        } else {
            return false;
        }
    }
    fn ToString(&self) -> String {
        let st: String;

        if let Some(ObjData) = STARGETRCREF_STARSERVICE!(self) {
            st = format!("{}", ObjData);
        } else {
            st = "service object is invalid".to_owned();
        }
        return st;
    }    

    fn Get(&self,Name: &Any) -> STARRESULT{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return None;
	        }
            let fbody : StarServiceBody;
            if let Some(ObjData) = STARGETRCREF_STARSERVICE!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return None;
            }         
        	let ParaName = SRPRustGetStr(Name, false);
        	let HashValue = Star_SRPI_GetHashValue(fbody.SRPInterface, ParaName as *mut c_void, vs_string_strlen(ParaName) as u32, 0);
	        if HashValue == 0x64756CF2 as u32 /*_Name*/ {
        		if vs_string_strcmp(ParaName, CString::new("_Name").unwrap().as_ptr()) == 0 {
		        	return Some(Box::new(SRPRustSetStr(Star_SRPI_GetServiceName(fbody.SRPInterface), false)));
		        }else{
                    return None;
                }
            }else if HashValue == 0x0D46C9D5 as u32 /*_ServiceGroup*/ {
        		if vs_string_strcmp(ParaName, CString::new("_ServiceGroup").unwrap().as_ptr()) == 0 {
        			return Some(Box::new(RustSRPQuerySrvGroup(fbody.ServiceGroupID)));
        		}else{
                    return None;
                }
            }else{
                return None;
            }
        }
	} 

    fn GetObject(&self,ObjectName: &Any) -> STAROBJECT {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let fbody : StarServiceBody;
            if let Some(ObjData) = STARGETRCREF_STARSERVICE!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return STARRC!(None);
            }          
        	let CharTemp = SRPRustGetStr(ObjectName, false);
        	if fbody.SRPInterface == 0 as *mut c_void {
		        STARRUST_SAFERELEASESTR(ObjectName, CharTemp);
		        return STARRC!(None);
	        }
	        let Object = Star_SRPI_GetObjectEx(fbody.SRPInterface, 0 as *mut c_void, CharTemp);
	        STARRUST_SAFERELEASESTR(ObjectName, CharTemp);
        	if Object == 0 as *mut c_void {
        		return STARRC!(None);
        	}
	        return ToStarObject(Object, fbody.SRPInterface as *mut c_void, false);
        }    
    }

    fn GetObjectEx(&self,ObjectID: &Any) -> STAROBJECT {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let fbody : StarServiceBody;
            if let Some(ObjData) = STARGETRCREF_STARSERVICE!(self) {    
                fbody = ObjData.ObjData.clone();
            }else{
                return STARRC!(None);
            }          
        	let CharTemp = SRPRustGetStr(ObjectID, false);
        	if fbody.SRPInterface == 0 as *mut c_void {
		        STARRUST_SAFERELEASESTR(ObjectID, CharTemp);
		        return STARRC!(None);
	        }        
            let mut LocalObjectID = VS_UUID::new();
	        Star_SRPI_StringToUuid(fbody.SRPInterface, CharTemp as *const i8, &mut LocalObjectID);
	        STARRUST_SAFERELEASESTR(ObjectID, CharTemp);
	        let Object = Star_SRPI_GetObject(fbody.SRPInterface, &mut LocalObjectID);
	        if Object == 0 as *mut c_void {
        		return STARRC!(None);
	        }
        	return ToStarObject(Object, fbody.SRPInterface as *mut c_void, false);
        }    
    }

    fn New(&self,args: &[&Any]) -> STAROBJECT {
    	let ObjData: &StarServiceBody;
        let argc : i32 = args.len() as i32;
    	let mut QueueAttrName : *mut c_char;
	    let mut AttributeChangeString : *mut c_char;
    	let mut ObjectNameString : *mut c_char;
	    let mut ObjectTemp : &Any;
    	let SRPObject : *mut c_void;
	    let mut SRPParentObject : *mut c_void;
    	let mut AttributeInfo : VS_ATTRIBUTEINFO = VS_ATTRIBUTEINFO::new();
	    let mut Index : i32 = 0;
    	let mut i : i32;
	    let AttributeNumber : i32;

        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return STARRC!(None);
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }    

        	QueueAttrName = 0 as *mut c_char;
        	SRPParentObject = 0 as *mut c_void;
        	AttributeChangeString = 0 as *mut c_char;
        	ObjectNameString = 0 as *mut c_char;
	        ObjectTemp = &(0 as isize);

        	if argc == 0 {
        		QueueAttrName = 0 as *mut c_char;
		        SRPParentObject = 0 as *mut c_void;
        	} else {
		        ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
		        if !AnyIsZero(ObjectTemp) {
                    if ObjectTemp.is::<String>() || ObjectTemp.is::<&str>() {
           				QueueAttrName = SRPRustGetStr(ObjectTemp,false);
           				Index = Index + 1;
	            		ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
        				if AnyIsZero(ObjectTemp) { //no more parameter
		        			ObjectNameString = QueueAttrName;
				        	QueueAttrName = 0 as *mut c_char;
					        SRPParentObject = 0 as *mut c_void;
        				} else {
		        			if IsStarObjectClassObject(ObjectTemp) == true {
				        		//SRPParentObject = RustObjectToSRPObject(ObjectTemp);
        						Index = Index + 1;
		        				ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
    					        if ObjectTemp.is::<String>() || ObjectTemp.is::<&str>() {
            						ObjectNameString = QueueAttrName;
            						QueueAttrName = 0 as *mut c_char;
            						SRPParentObject =  0 as *mut c_void;
						            AttributeChangeString = SRPRustGetStr(ObjectTemp, true);
                                    Index = Index + 1;
            						ObjectTemp = &(0 as isize); //end
					            } else {
						            return STARRC!(None);
        					    }
                            }
                        }
        			} else if IsStarObjectClassObject(ObjectTemp) == true {
        				SRPParentObject = RustObjectToSRPObject(ObjectTemp);
				        Index = Index + 1;
				        ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
        			} else {
		        		Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
        				return STARRC!(None);
        			}
        		} else {
			        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
			        return STARRC!(None);
		        }
	        }
	        if !AnyIsZero(ObjectTemp) {
		        if ObjectTemp.is::<String>() || ObjectTemp.is::<&str>() {
        			ObjectNameString = SRPRustGetStr(ObjectTemp, false);
        			Index = Index + 1;
        			ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
			        if AnyIsZero(ObjectTemp) { //no more parameter
        			} else if ObjectTemp.is::<String>() || ObjectTemp.is::<&str>() {
        				AttributeChangeString = SRPRustGetStr(ObjectTemp, true);
        			}
        		}
	        }

	        if SRPParentObject == 0 as *mut c_void {
		        SRPObject = Star_SRPI_MallocObjectL(ObjData.SRPInterface, 0 as *mut VS_UUID, 0, 0 as *mut c_void);
        	} else {
		        if Star_SRPI_IsObject(ObjData.SRPInterface, SRPParentObject) == VS_TRUE {
        			if QueueAttrName != 0 as *mut c_char {
				        if Star_SRPI_GetAttributeInfoEx(ObjData.SRPInterface, Star_SRPI_GetClass(ObjData.SRPInterface, SRPParentObject), QueueAttrName, &mut AttributeInfo) == VS_FALSE {
                            RustPrintError(
                                VSFAULT_WARNING,
                                CString::new(format!("Get Parent Attribute [{}]",SRPRustSetStr(QueueAttrName, false)))
                                    .unwrap()
                                    .as_ptr()
                                );                            
					        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
					        return STARRC!(None);
			            }
		            } else {
			            AttributeNumber = Star_SRPI_GetAttributeNumber(ObjData.SRPInterface, Star_SRPI_GetClass(ObjData.SRPInterface, SRPParentObject));
           				i = 0;
           				loop {
		            		if i >= AttributeNumber {
					            break;
				            }
				            Star_SRPI_GetAttributeInfo(ObjData.SRPInterface, Star_SRPI_GetClass(ObjData.SRPInterface, SRPParentObject), i as u8, &mut AttributeInfo);
           					if AttributeInfo.Type == VSTYPE_PTR && AttributeInfo.SyncType == VS_TRUE && starrust_uuidisvalid(&AttributeInfo.StructID) == VS_TRUE {
					            break;
				            }
				            i = i + 1;
			            }
			            if i >= AttributeNumber {
                            RustPrintError(
                                VSFAULT_WARNING,
                                CString::new(format!("not Found Parent Sync Attribute Queue"))
                                    .unwrap()
                                    .as_ptr()
                                );                            
				            Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
				            return STARRC!(None);
                        }
		            }
		            SRPObject = Star_SRPI_MallocObject(ObjData.SRPInterface, SRPParentObject, AttributeInfo.AttributeIndex, 0 as *mut VS_UUID, 0, 0 as *mut c_void);
           		} else {
		            SRPObject = Star_SRPI_MallocObject(ObjData.SRPInterface, SRPParentObject, 0, 0 as *mut VS_UUID, 0, 0 as *mut c_void);
           		}
            }
	        if SRPObject == 0 as *mut c_void {
        		Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
        		return STARRC!(None);
        	}
	        Star_SRPI_SetSourceScript(ObjData.SRPInterface, SRPObject, StarRust_ScriptInterfaceIndex);
	        if ObjectNameString != 0 as *mut c_char && vs_string_strlen(ObjectNameString) != 0 {
		        Star_SRPI_SetName(ObjData.SRPInterface, SRPObject, ObjectNameString);
	        }
	        if AttributeChangeString !=  0 as *mut c_char && vs_string_strlen(AttributeChangeString) != 0 {
        		Star_SRPI_LuaInitObject(ObjData.SRPInterface, SRPObject, AttributeChangeString);
	        }
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
	        return ToStarObject(SRPObject, ObjData.SRPInterface as *mut c_void,true);
        }
    }

    fn RunScript(&self,In_ScriptInterface: &Any, In_ScriptBuf: &Any, In_Name: &Any, In_WorkDirectory: &Any) -> (bool,String) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return (false,"".to_owned());
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        } 

        	let ScriptBuf = SRPRustGetStr(In_ScriptBuf, true);
	        if ScriptBuf == 0 as *mut c_char {
		        return (false,"".to_owned());
	        }
	        let Name = SRPRustGetStr(In_Name, false);
	        let WorkDirectory = SRPRustGetStr(In_WorkDirectory, true);
	        let ScriptInterface = SRPRustGetStr(In_ScriptInterface, false);
            let mut ErrorInfo : Vec<*mut c_char> = Vec::new();
            ErrorInfo.push(0 as *mut c_char);

	        let ResultVal = Star_SRPI_DoBuffer(ObjData.SRPInterface, ScriptInterface, ScriptBuf as *const i8, vs_string_strlen(ScriptBuf) as i32, Name,ErrorInfo.as_mut_ptr() as *mut [*mut c_char; 1], WorkDirectory, VS_FALSE);
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface,ScriptBuf as *mut c_void);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, WorkDirectory as *mut c_void);
        	if ResultVal == VS_FALSE {
            	STARRUST_SAFERELEASESTR(In_Name, Name);
	            STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);          
                return (false,SRPRustSetStr(ErrorInfo[0], true))
        	} else {
            	STARRUST_SAFERELEASESTR(In_Name, Name);
	            STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);          
                return (true,"".to_owned())                ;
            }
        }
    }

    fn RunScriptEx(&self,In_ScriptInterface: &Any, BinBuf: &STARBINBUF, In_Name: &Any, In_WorkDirectory: &Any) -> (bool,String) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return (false,"".to_owned());
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }         

        	let SRPBinBufInterface = RustObjectToBinBuf(BinBuf);
	        if SRPBinBufInterface == 0 as *mut c_void || Star_SRPBinBuf_GetBufPtr(SRPBinBufInterface, 0) == 0 as *mut i8 {
        		return (false,"".to_owned());
	        }
	        let Name = SRPRustGetStr(In_Name, false);
        	let WorkDirectory = SRPRustGetStr(In_WorkDirectory, true);
	        let ScriptInterface = SRPRustGetStr(In_ScriptInterface, false);

            let mut ErrorInfo : Vec<*mut c_char> = Vec::new();
            ErrorInfo.push(0 as *mut c_char);

	        let ResultVal = Star_SRPI_DoBuffer(ObjData.SRPInterface, ScriptInterface, Star_SRPBinBuf_GetBufPtr(SRPBinBufInterface, 0), Star_SRPBinBuf_GetOffset(SRPBinBufInterface) as i32, Name,ErrorInfo.as_mut_ptr() as *mut [*mut c_char; 1], WorkDirectory, VS_FALSE);
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface,WorkDirectory as *mut c_void);
	        STARRUST_SAFERELEASESTR(In_Name, Name);
	        STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);
        	if ResultVal == VS_FALSE {
            	STARRUST_SAFERELEASESTR(In_Name, Name);
	            STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);          
                return (false,SRPRustSetStr(ErrorInfo[0], true))
        	} else {
            	STARRUST_SAFERELEASESTR(In_Name, Name);
	            STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);          
                return (true,"".to_owned())                ;
            }
        }
    }    

    fn DoFile(&self,In_ScriptInterface: &Any, In_FileName: &Any, In_WorkDirectory: &Any) -> (bool,String) {
	    return self.DoFileEx(In_ScriptInterface, In_FileName, In_WorkDirectory, &"");
    }

    fn DoFileEx(&self,In_ScriptInterface: &Any, In_FileName: &Any, In_WorkDirectory: &Any, In_ModuleName: &Any)-> (bool,String) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return (false,"".to_owned());
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        } 
        	let FileName = SRPRustGetStr(In_FileName, true);
        	if FileName == 0 as *mut c_char {
		         return (false,"".to_owned());
	        }
        	let WorkDirectory = SRPRustGetStr(In_WorkDirectory, true);
	        let ScriptInterface = SRPRustGetStr(In_ScriptInterface, false);
	        let ModuleName = SRPRustGetStr(In_ModuleName, false);

            let mut ErrorInfo : Vec<*mut c_char> = Vec::new();
            ErrorInfo.push(0 as *mut c_char);

	        let ResultVal = Star_SRPI_DoFileEx(ObjData.SRPInterface, ScriptInterface, FileName, ErrorInfo.as_mut_ptr() as *mut [*mut c_char; 1], WorkDirectory, VS_FALSE, ModuleName);
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, FileName as *mut c_void);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, WorkDirectory as *mut c_void);
	        STARRUST_SAFERELEASESTR(In_ScriptInterface, ScriptInterface);
	        STARRUST_SAFERELEASESTR(In_ModuleName, ModuleName);
        	if ResultVal == VS_FALSE {
                return (false,SRPRustSetStr(ErrorInfo[0], true))
        	} else {
                return (true,"".to_owned())                ;
            }            
        }
    }

    fn IsServiceRegistered(&self) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return false;
	        } 
        	return FROMVS_BOOL(Star_SRPI_IsRegistered(ObjData.SRPInterface));
        }
    }

    fn CheckPassword(&self,Flag: bool) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return;
	        }         
        	Star_SRPI_CheckPassword(ObjData.SRPInterface, TOVS_BOOL(Flag));
        }    
    }

    fn LoadRawModule(&self,ScriptInterface: &Any, ModuleName: &Any, FileOrString: &Any, IsString: bool) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return false;
	        }         
	        let In_ScriptInterface = SRPRustGetStr(ScriptInterface, false);
	        let In_FileOrString = SRPRustGetStr(FileOrString, true);
	        let In_ModuleName = SRPRustGetStr(ModuleName, true);

	        let ResultVal = Star_SRPI_LoadRawModule(ObjData.SRPInterface, In_ScriptInterface, In_ModuleName, In_FileOrString, TOVS_BOOL(IsString), 0 as *mut [*mut c_char;1]);
        	STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, In_FileOrString as *mut c_void);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, In_ModuleName  as *mut c_void);
	        return FROMVS_BOOL(ResultVal);
        }    
    }

    fn NewRawProxy(&self,ScriptInterface: &Any, AttachObject: &STAROBJECT, AttachFunction: &Any, ProyInfo: &Any, ProxyType: isize) -> STAROBJECT {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return STARRC!(None);
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }           
	        let In_ScriptInterface = SRPRustGetStr(ScriptInterface, false);
    	    let In_ProyInfo = SRPRustGetStr(ProyInfo, false);
	        let In_AttachFunction = SRPRustGetStr(AttachFunction, false);
    		let SRPObject = RustObjectToSRPObject(AttachObject);
		    if SRPObject == 0 as *mut c_void {
    			STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
	    		STARRUST_SAFERELEASESTR(ProyInfo, In_ProyInfo);
		    	STARRUST_SAFERELEASESTR(AttachFunction, In_AttachFunction);
			    return STARRC!(None);
		    }
		    let ResultVal = Star_SRPI_NewRawProxy(ObjData.SRPInterface, In_ScriptInterface, SRPObject, In_AttachFunction, In_ProyInfo, ProxyType as i32);
        	STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
	        STARRUST_SAFERELEASESTR(ProyInfo, In_ProyInfo);
	        STARRUST_SAFERELEASESTR(AttachFunction, In_AttachFunction);
	        if ResultVal == 0 as *mut c_void {
		        return STARRC!(None);
	        }
	        return ToStarObject(ResultVal, ObjData.SRPInterface as *mut c_void, true);
        }
    }

    fn ImportRawContext(&self,ScriptInterface: &Any, ContextName: &Any, IsClass: bool, ContextInfo: &Any) -> STAROBJECT {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return STARRC!(None);
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }        
	        let In_ScriptInterface = SRPRustGetStr(ScriptInterface, false);
	        let In_ContextName = SRPRustGetStr(ContextName, false);
	        let In_ContextInfo = SRPRustGetStr(ContextInfo, false);
	        let ResultVal = Star_SRPI_ImportRawContext(ObjData.SRPInterface, In_ScriptInterface, In_ContextName, TOVS_BOOL(IsClass), In_ContextInfo);
	        STARRUST_SAFERELEASESTR(ScriptInterface, In_ScriptInterface);
	        STARRUST_SAFERELEASESTR(ContextName, In_ContextName);
	        STARRUST_SAFERELEASESTR(ContextInfo, In_ContextInfo);
	        if ResultVal == 0 as *mut c_void {
		        return STARRC!(None);
	        }
	        return ToStarObject(ResultVal, ObjData.SRPInterface as *mut c_void, true);
        }
    }

    fn GetLastError(&self) -> isize {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return 0;
	        }            
        	return Star_SRPI_GetLastError(ObjData.SRPInterface) as isize;
        }
    }

    fn GetLastErrorInfo(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarServiceBody;
            if let Some(fbody) = STARGETRCREF_STARSERVICE!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SRPInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }    
            let mut LineIndex : u32 = 0;                    
            let mut SourceName : Vec<*mut c_char> = Vec::new();
            SourceName.push(0 as *mut c_char);
	        let TextBuf = Star_SRPI_GetLastErrorInfo(ObjData.SRPInterface, &mut LineIndex,SourceName.as_mut_ptr() as usize as *mut [*mut c_char;1]);
	        return format!("[{}:{}]{}", SRPRustSetStr(SourceName[0],false), LineIndex, SRPRustSetStr(TextBuf, true));
        }
    }
}    


fn SRPObject_GetArrayObject<'a>(argc: i32, args: &'a [&Any], Index: i32) -> &'a Any {
   	if Index >= argc {
    	return &(0 as isize);
   	}
    return args[Index as usize];
}    

fn AnyIsZero(val:&Any) -> bool{
    if let Some(ObjData) = val.downcast_ref::<isize>() {
        if *ObjData == 0 {
            return true;
        }else{
            return false;
        }
    }else{
        return false;
    }
}


/*----------------------------------------------------------------------------*/
/* StarParaPkg */
/*----------------------------------------------------------------------------*/
fn StarParaPkg_FromTuple_Sub(tuple: &[&Any], ParaPkg: *mut c_void, BasicSRPInterface: *const c_void,StartIndex: i32, SRPInterface: *const c_void) -> bool 
{
    unsafe{
        let mut Index: i32;

	    if StartIndex == 0 && tuple.len() == 0 {
		    Star_SRPParaPkg_Clear(ParaPkg);
		    return true;
	    }
	    if StartIndex >= tuple.len() as i32 {
		    Star_SRPParaPkg_Clear(ParaPkg);
		    return false;
	    }
	    if tuple.len() == 1 {
            if let Some(ObjData) = tuple[0].downcast_ref::<HashMap<String,&Any>>() {
			    /*  as dict */
                Index = 0;
                let mut NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
                for (key, value) in ObjData.iter() {
                    let (CharLen,CharTemp) = SRPRustGetStrEx(key, false);
                    Star_SRPParaPkg_InsertStrEx(ParaPkg, Index, CharTemp, CharLen as u32);
                    STARRUST_SAFERELEASESTR(key, CharTemp);
   
                    Star_SRPParaPkg_Clear(NewParaPkg);
                    let s = [*value];
				    if StarParaPkg_FromTuple_Sub(&s, NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
					    Star_SRPParaPkg_Clear(ParaPkg);
					    return false;
				    }
				    Star_SRPParaPkg_AppendFrom(ParaPkg, NewParaPkg);
				    Index = Index + 2;
			    }
			    Star_SRPParaPkg_AsDict(ParaPkg, VS_TRUE);
			    Star_SRPParaPkg_Release(NewParaPkg);
			    return true;
		    }else if let Some(ObjData) = tuple[0].downcast_ref::<HashMap<&str,&Any>>() {
			    /*  as dict */
                Index = 0;
                let mut NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
                for (key, value) in ObjData.iter() {
                    let (CharLen,CharTemp) = SRPRustGetStrEx(key, false);
                    Star_SRPParaPkg_InsertStrEx(ParaPkg, Index, CharTemp, CharLen as u32);
                    STARRUST_SAFERELEASESTR(key, CharTemp);
   
                    Star_SRPParaPkg_Clear(NewParaPkg);
                    let s = [*value];
				    if StarParaPkg_FromTuple_Sub(&s, NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
					    Star_SRPParaPkg_Clear(ParaPkg);
					    return false;
				    }
				    Star_SRPParaPkg_AppendFrom(ParaPkg, NewParaPkg);
				    Index = Index + 2;
			    }
			    Star_SRPParaPkg_AsDict(ParaPkg, VS_TRUE);
			    Star_SRPParaPkg_Release(NewParaPkg);
			    return true;
            }
        }
	    for i in 0..tuple.len() {
            let Index = i as i32;
		    if Index < StartIndex {
			    continue;
            }
            if let Some(fdata) = tuple[i].downcast_ref::<bool>() {
                Star_SRPParaPkg_InsertBool(ParaPkg, Index-StartIndex, TOVS_BOOL(*fdata));
            }else if let Some(fdata) = tuple[i].downcast_ref::<i8>() {
                Star_SRPParaPkg_InsertInt(ParaPkg, Index-StartIndex, *fdata as i32);
            }else if let Some(fdata) = tuple[i].downcast_ref::<i16>() {
                Star_SRPParaPkg_InsertInt(ParaPkg, Index-StartIndex, *fdata as i32);  
            }else if let Some(fdata) = tuple[i].downcast_ref::<i32>() {
                Star_SRPParaPkg_InsertInt(ParaPkg, Index-StartIndex, *fdata as i32);          
            }else if let Some(fdata) = tuple[i].downcast_ref::<i64>() {
                Star_SRPParaPkg_InsertInt64(ParaPkg, Index-StartIndex, *fdata as i64);     
            }else if let Some(fdata) = tuple[i].downcast_ref::<isize>() {
                if size_of::<isize>() == 4 {
                    Star_SRPParaPkg_InsertInt(ParaPkg, Index-StartIndex, *fdata as i32);                                                        
                }else{
                    Star_SRPParaPkg_InsertInt64(ParaPkg, Index-StartIndex, *fdata as i64);
                }
            }else if let Some(fdata) = tuple[i].downcast_ref::<u8>() {
                Star_SRPParaPkg_InsertInt(ParaPkg, Index-StartIndex, *fdata as i32);
            }else if let Some(fdata) = tuple[i].downcast_ref::<u16>() {
                Star_SRPParaPkg_InsertInt(ParaPkg, Index-StartIndex, *fdata as i32);  
            }else if let Some(fdata) = tuple[i].downcast_ref::<u32>() {
                Star_SRPParaPkg_InsertInt(ParaPkg, Index-StartIndex, *fdata as i32);          
            }else if let Some(fdata) = tuple[i].downcast_ref::<u64>() {
                Star_SRPParaPkg_InsertInt64(ParaPkg, Index-StartIndex, *fdata as i64);     
            }else if let Some(fdata) = tuple[i].downcast_ref::<usize>() {
                if size_of::<usize>() == 4 {
                    Star_SRPParaPkg_InsertInt(ParaPkg, Index-StartIndex, *fdata as i32);                                                        
                }else{
                    Star_SRPParaPkg_InsertInt64(ParaPkg, Index-StartIndex, *fdata as i64);
                }     
            }else if let Some(fdata) = tuple[i].downcast_ref::<f32>() {
                Star_SRPParaPkg_InsertFloat(ParaPkg, Index-StartIndex, *fdata as f64);          
            }else if let Some(fdata) = tuple[i].downcast_ref::<f64>() {
                Star_SRPParaPkg_InsertFloat(ParaPkg, Index-StartIndex, *fdata as f64);        
            }else if let Some(fdata) = tuple[i].downcast_ref::<String>() {
                let (slen,cstr) = SRPRustGetStrEx(fdata, false);
                Star_SRPParaPkg_InsertStrEx(ParaPkg, Index-StartIndex, cstr,slen as u32);
                STARRUST_SAFERELEASESTR(fdata, cstr);
            }else if let Some(fdata) = tuple[i].downcast_ref::<&str>() {
                let (slen,cstr) = SRPRustGetStrEx(fdata, false);
                Star_SRPParaPkg_InsertStrEx(ParaPkg, Index-StartIndex, cstr,slen as u32);
                STARRUST_SAFERELEASESTR(fdata, cstr); 
            }else if let Some(fdata) = tuple[i].downcast_ref::<STARBINBUF>() {
			    let SRPBinBufInterface = RustObjectToBinBuf(fdata);
			    if SRPBinBufInterface != 0 as *mut c_void {
				    Star_SRPParaPkg_InsertBinEx(ParaPkg, Index-StartIndex, Star_SRPBinBuf_GetBuf(SRPBinBufInterface), Star_SRPBinBuf_GetOffset(SRPBinBufInterface) as i32, Star_SRPBinBuf_IsFromRaw(SRPBinBufInterface));
    			} else {
	    			Star_SRPParaPkg_Clear(ParaPkg);
		    		return false;
			    }               
            }else if let Some(fdata) = tuple[i].downcast_ref::<STARPARAPKG>() {
                let NewParaPkg = RustObjectToParaPkg(fdata);
    			if NewParaPkg != 0 as *mut c_void {
	    			Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
		    	} else {
			    	Star_SRPParaPkg_Clear(ParaPkg);
				    return false
			    }       
            }else if let Some(fdata) = tuple[i].downcast_ref::<STAROBJECT>() {
			    let SRPObject = RustObjectToSRPObject(fdata);
			    Star_SRPParaPkg_InsertObject(ParaPkg, Index-StartIndex, SRPObject);   
/* may be not supported later                  
            }else if let Some(fdata) = tuple[i].downcast_ref::<[&Any]>() {
                let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
--*/
            }else if let Some(fdata) = tuple[i].downcast_ref::<HashMap<String,&Any>>() {                
    			let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
                let newval = tuple[i];
	    		if StarParaPkg_FromTuple_Sub(&[newval], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
		    		Star_SRPParaPkg_Release(NewParaPkg);
			    	Star_SRPParaPkg_Clear(ParaPkg);
				    return false;
    			}
		    	Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
	    		Star_SRPParaPkg_Release(NewParaPkg);
            }else if let Some(fdata) = tuple[i].downcast_ref::<HashMap<&str,&Any>>() {                
    			let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
                let newval = tuple[i];
	    		if StarParaPkg_FromTuple_Sub(&[newval], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
		    		Star_SRPParaPkg_Release(NewParaPkg);
			    	Star_SRPParaPkg_Clear(ParaPkg);
				    return false;
    			}
		    	Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
	    		Star_SRPParaPkg_Release(NewParaPkg);   
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<bool>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertBool(NewParaPkg, i as i32, TOVS_BOOL(fdata[i]));
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);   
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<i8>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertInt(NewParaPkg, i as i32, fdata[i] as i32);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);     
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<u8>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertInt(NewParaPkg, i as i32, fdata[i] as i32);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);     
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<i16>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertInt(NewParaPkg, i as i32, fdata[i] as i32);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);       
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<u16>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertInt(NewParaPkg, i as i32, fdata[i] as i32);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);    
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<i32>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertInt(NewParaPkg, i as i32, fdata[i] as i32);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);          
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<u32>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertInt(NewParaPkg, i as i32, fdata[i] as i32);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);   
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<i64>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertInt64(NewParaPkg, i as i32, fdata[i] as i64);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);          
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<u64>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertInt64(NewParaPkg, i as i32, fdata[i] as i64);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);  
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<isize>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
                    if size_of::<usize>() == 4 {
                        Star_SRPParaPkg_InsertInt(NewParaPkg, i as i32, fdata[i] as i32);
                    }else{
				        Star_SRPParaPkg_InsertInt64(NewParaPkg, i as i32, fdata[i] as i64);
                    }
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);      
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<usize>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
                    if size_of::<usize>() == 4 {
                        Star_SRPParaPkg_InsertInt(NewParaPkg, i as i32, fdata[i] as i32);
                    }else{
				        Star_SRPParaPkg_InsertInt64(NewParaPkg, i as i32, fdata[i] as i64);
                    }
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);   
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<f32>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertFloat(NewParaPkg, i as i32, fdata[i] as f64);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);   
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<f64>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
				    Star_SRPParaPkg_InsertFloat(NewParaPkg, i as i32, fdata[i] as f64);
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);            
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<String>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
                    let (slen,cstr) = SRPRustGetStrEx(fdata, false);
                    Star_SRPParaPkg_InsertStrEx(NewParaPkg, i as i32, cstr,slen as u32);
                    STARRUST_SAFERELEASESTR(fdata, cstr); 
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);            
            }else if let Some(fdata) = tuple[i].downcast_ref::<Vec<&str>>() {  
			    let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
			    for i in 0..fdata.len() {
                    let (slen,cstr) = SRPRustGetStrEx(fdata, false);
                    Star_SRPParaPkg_InsertStrEx(NewParaPkg, i as i32, cstr,slen as u32);
                    STARRUST_SAFERELEASESTR(fdata, cstr); 
			    }
			    Star_SRPParaPkg_InsertParaPackage(ParaPkg, Index-StartIndex, NewParaPkg);
			    Star_SRPParaPkg_Release(NewParaPkg);                                                                                                                                                                                                                  
            }else{
				Star_SRPParaPkg_Clear(ParaPkg);
				return false;
            }           
		}        
	}
	return true;
}

fn StarParaPkg_ToTuple_Sub(ParaPkg: *const c_void, BasicSRPInterface: *const c_void, StartIndex: i32, ToRaw: bool) -> STARRESULT_TUPLE {
    let Index : i32;
    let Number : i32;
    let mut RetValue : STARRESULT_TUPLE = Vec::new();
    let SRPBinBufInterface : *const c_void;
    let SRPParaPackageInterface : *const c_void;
    let Length : i32;
    let Buf : *const c_void;

    unsafe{
        Number = Star_SRPParaPkg_GetNumber(ParaPkg);
        if Number < StartIndex {
	        return RetValue;
        }
       	for Index in StartIndex..Number {
	        match Star_SRPParaPkg_GetType(ParaPkg, Index) {
   		    SRPPARATYPE_INT => RetValue.push(Some(Box::new(Star_SRPParaPkg_GetInt(ParaPkg, Index) as i32))),
	        SRPPARATYPE_INT64 => RetValue.push(Some(Box::new(Star_SRPParaPkg_GetInt64(ParaPkg, Index) as i64))),
       		SRPPARATYPE_FLOAT => RetValue.push(Some(Box::new(Star_SRPParaPkg_GetFloat(ParaPkg, Index) as f64))),
       		SRPPARATYPE_CHARPTR => 
		    {
   				let Str : *mut c_char;
    			let mut StrLen : u32 = 0;
	    		Str = Star_SRPParaPkg_GetStrEx(ParaPkg, Index, &mut StrLen);
		    	let ttt = SRPRustSetStrEx(Str, StrLen as i32, true);
   				if ttt.len() == 0 && StrLen != 0 { //--changed to byte array
    				let mut Temp : Vec<u8> = Vec::with_capacity(StrLen as usize);                    
	    			if StrLen != 0 {
                        Temp.resize(StrLen as usize,0);
		    			vs_memcpy(Temp.as_mut_ptr() as *mut c_void, Str as *const c_void, StrLen as isize);
			    	}
				    RetValue.push(Some(Box::new(Temp)));
   				} else {
    				RetValue.push(Some(Box::new(ttt)));
	    		}
		    },
	        SRPPARATYPE_BOOL => RetValue.push(Some(Box::new(FROMVS_BOOL(Star_SRPParaPkg_GetBool(ParaPkg, Index))))),
	        SRPPARATYPE_BIN => 
		    {
			    let mut FromRaw : VS_BOOL = VS_FALSE;
                let mut Length : i32 = 0;
			    let Buf = Star_SRPParaPkg_GetBinEx(ParaPkg, Index, &mut Length, &mut FromRaw) as *const c_void;
			    if ToRaw == false {
				    if FromRaw == VS_FALSE {
					    let SRPBinBufInterface = Star_SRPBasic_GetSRPBinBufInterface(BasicSRPInterface);
					    Star_SRPBinBuf_Set(SRPBinBufInterface, 0, Length as u32, Buf as *mut i8);
					    RetValue.push(Some(Box::new(ToStarBinBuf(SRPBinBufInterface, Star_SRPBasic_GetServiceGroupID(BasicSRPInterface), true))));
				    } else {
                        let mut Temp : Vec<u8> = Vec::with_capacity(Length as usize);
   						if Length != 0 {
                            Temp.resize(Length as usize,0);
                            vs_memcpy(Temp.as_mut_ptr() as *mut c_void, Buf as *const c_void, Length as isize);
	    				}
   						RetValue.push(Some(Box::new(Temp)));
				    }
			    } else {
   					let mut Temp : Vec<u8> = Vec::with_capacity(Length as usize);
   					if Length != 0 {
                        Temp.resize(Length as usize,0);
                        vs_memcpy(Temp.as_mut_ptr() as *mut c_void, Buf as *const c_void, Length as isize);
	    			}
   					RetValue.push(Some(Box::new(Temp)));
		        }
   			},
	        SRPPARATYPE_OBJECT =>
		    {
			    let SRPObject : *mut c_void = Star_SRPParaPkg_GetObject(ParaPkg, Index);
			    if SRPObject == 0 as *mut c_void {
				    RetValue.push(None);
			    } else {
				    RetValue.push(Some(Box::new(ToStarObject(SRPObject, BasicSRPInterface as *mut c_void, false))));
				}
			},
       		SRPPARATYPE_PARAPKG =>
            {
		        let SRPParaPackageInterface = Star_SRPParaPkg_GetParaPackage(ParaPkg, Index);
		        if SRPParaPackageInterface == 0 as *mut c_void {
			        RetValue.push(None);
		        } else {
   					RetValue.push(Some(Box::new(StarParaPkg_ToTuple_Sub(SRPParaPackageInterface, BasicSRPInterface, 0, ToRaw))));
                }
			},
            _ => RetValue.push(None),
            }
   		}
        return RetValue;
    }
}

pub trait STARPARAPKG_TRAIT {
    fn IsValid(&self) -> bool;
    fn ToString(&self) -> String;

    fn GetNumber(&self) -> isize;
    fn Get(&self,Index: isize) -> STARRESULT;
    fn GetBool(&self,Index:isize) -> bool;
    fn GetInt(&self,Index:isize) -> i32;
    fn GetInt64(&self,Index:isize) -> i64;
    fn GetString(&self,Index:isize) -> String;
    fn GetDouble(&self,Index:isize) -> f64;
    fn GetObject(&self,Index:isize)->STAROBJECT;
    fn GetParaPkg(&self,Index:isize) -> STARPARAPKG;
    fn GetBinBuf(&self,Index:isize) -> STARBINBUF; 
    fn Clear<'a>(&'a self) -> &'a STARPARAPKG;
    fn AppendFrom(&self,SrcParaPkg: &STARPARAPKG) -> bool;
    fn Set<'a>(&'a self,Index: isize, Value: &Any) -> &'a STARPARAPKG;
    fn Build<'a>(&'a self,tuple: &[&Any]) -> &'a STARPARAPKG;
    fn Free(&self);
    fn Dispose(&self);
    fn ReleaseOwner(&self);
    fn AsDict<'a>(&'a self,IsDict: bool) -> &'a STARPARAPKG;
    fn IsDict(&self) -> bool;
    fn FromJSon(&self,Buf: &Any) -> bool;
    fn ToJSon(&self) -> String;

    fn FromVecBool<'a>(&'a self,val:&Vec<bool>) -> &'a STARPARAPKG;
    fn FromVecI8<'a>(&'a self,val:&Vec<i8>) -> &'a STARPARAPKG;
    fn FromVecU8<'a>(&'a self,val:&Vec<u8>) -> &'a STARPARAPKG;
    fn FromVecI16<'a>(&'a self,val:&Vec<i16>) -> &'a STARPARAPKG;
    fn FromVecU16<'a>(&'a self,val:&Vec<u16>) -> &'a STARPARAPKG;
    fn FromVecI32<'a>(&'a self,val:&Vec<i32>) -> &'a STARPARAPKG;
    fn FromVecU32<'a>(&'a self,val:&Vec<u32>) -> &'a STARPARAPKG;
    fn FromVecI64<'a>(&'a self,val:&Vec<i64>) -> &'a STARPARAPKG;
    fn FromVecU64<'a>(&'a self,val:&Vec<u64>) -> &'a STARPARAPKG;
    fn FromVecISize<'a>(&'a self,val:&Vec<isize>) -> &'a STARPARAPKG;
    fn FromVecUSize<'a>(&'a self,val:&Vec<usize>) -> &'a STARPARAPKG;
    fn FromVecF32<'a>(&'a self,val:&Vec<f32>) -> &'a STARPARAPKG;
    fn FromVecF64<'a>(&'a self,val:&Vec<f64>) -> &'a STARPARAPKG;
    fn FromVecString<'a>(&'a self,val:&Vec<String>) -> &'a STARPARAPKG;
    fn FromVecStr<'a>(&'a self,val:&Vec<&'static str>) -> &'a STARPARAPKG;

    fn ToVecBool(&self) -> Vec<bool>;
    fn ToVecInt(&self) -> Vec<i32>;
    fn ToVecInt64(&self) -> Vec<i64>;
    fn ToVecString(&self) -> Vec<String>;
    fn ToVecDouble(&self) -> Vec<f64>;

}

impl STARPARAPKG_TRAIT for STARPARAPKG {
    fn IsValid(&self) -> bool {
        if let Some(ObjData) = STARGETRCREF_STARPARAPKG!(self) {
            return ObjData.IsValid;
        } else {
            return false;
        }
    }
    fn ToString(&self) -> String {
        let st: String;

        if let Some(ObjData) = STARGETRCREF_STARPARAPKG!(self) {
            st = format!("{}", ObjData);
        } else {
            st = "parapkg object is invalid".to_owned();
        }
        return st;
    }     

    fn GetNumber(&self) -> isize {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return 0;
	        }          
        	return Star_SRPParaPkg_GetNumber(ObjData.ParaPkg) as isize;
        }
    }

    fn Get(&self,Index: isize) -> STARRESULT {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return None;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return None;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return None;
	        }           
        	if true {
		        match Star_SRPParaPkg_GetType(ObjData.ParaPkg, Index as i32) {
		        SRPPARATYPE_BOOL => return Some(Box::new(FROMVS_BOOL(Star_SRPParaPkg_GetBool(ObjData.ParaPkg, Index as i32)))),
        		SRPPARATYPE_INT => return Some(Box::new(Star_SRPParaPkg_GetInt(ObjData.ParaPkg,Index as i32))),
		        SRPPARATYPE_INT64 => return Some(Box::new(Star_SRPParaPkg_GetInt64(ObjData.ParaPkg, Index as i32))),
		        SRPPARATYPE_FLOAT => return Some(Box::new(Star_SRPParaPkg_GetFloat(ObjData.ParaPkg, Index as i32))),
		        SRPPARATYPE_BIN =>
			    {
				    let mut FromRaw : VS_BOOL = VS_FALSE;
				    let BasicSRPInterface = RustSRPGetBasicSRPInterface(ObjData.ServiceGroupID);
				    if BasicSRPInterface == 0 as *mut c_void {
					    return None;
				    }
                    let mut Length : i32 = 0;
				    let Buf = Star_SRPParaPkg_GetBinEx(ObjData.ParaPkg, Index as i32,&mut Length, &mut FromRaw);
				    if Buf == 0 as *mut i8 {
					    return None;
				    }
				    if FromRaw == VS_FALSE {
					    let SRPBinBufInterface = Star_SRPBasic_GetSRPBinBufInterface(BasicSRPInterface);
					    Star_SRPBinBuf_Set(SRPBinBufInterface, 0, Length as u32, Buf as *mut i8);
					    return Some(Box::new(ToStarBinBuf(SRPBinBufInterface, ObjData.ServiceGroupID, true)));
				    } else {
   					    let mut Temp : Vec<u8> = Vec::with_capacity(Length as usize);
   					    if Length != 0 {
                            Temp.resize(Length as usize,0);
                            vs_memcpy(Temp.as_mut_ptr() as *mut c_void, Buf as *const c_void, Length as isize);
	    			    }                        
    					return Some(Box::new(Temp));
				    }
			    },
		        SRPPARATYPE_CHARPTR =>
			    {
    				let mut StrLen: u32 = 0;
				    let Str = Star_SRPParaPkg_GetStrEx(ObjData.ParaPkg, Index as i32, &mut StrLen);
				    let ttt = SRPRustSetStrEx(Str, StrLen as i32, true);
				    if ttt.len() != 0 || StrLen == 0 {
					    return Some(Box::new(ttt));
				    }
				    let mut Temp : Vec<u8> = Vec::with_capacity(StrLen as usize);
				    if StrLen != 0 {
                        Temp.resize(StrLen as usize,0);
                        vs_memcpy(Temp.as_mut_ptr() as *mut c_void, Str as *const c_void, StrLen as isize);
    			    }
    				return Some(Box::new(Temp));
			    },
		        SRPPARATYPE_OBJECT =>
                {
			        let BasicSRPInterface = RustSRPGetBasicSRPInterface(ObjData.ServiceGroupID);
			        if BasicSRPInterface == 0 as *mut c_void {
				        return None;
			        }
			        let SRPObject = Star_SRPParaPkg_GetObject(ObjData.ParaPkg,Index as i32);
			        if SRPObject == 0 as *mut c_void {
				        return None;
			        }
			        return Some(Box::new(ToStarObject_Basic(SRPObject, BasicSRPInterface, false)));
                },
		        SRPPARATYPE_PARAPKG => 
                {
			        let SRPParaPackageInterface = Star_SRPParaPkg_GetParaPackage(ObjData.ParaPkg, Index as i32);
			        if SRPParaPackageInterface == 0 as *mut c_void {
				        return None;
			        }
			        Star_SRPParaPkg_AddRef(SRPParaPackageInterface);
			        return Some(Box::new(ToStarParaPkg(SRPParaPackageInterface, ObjData.ServiceGroupID, true)));
                },
		        SRPPARATYPE_INVALID => return None,
                _ => return None,
                }
    		}
		    return None;
        }
	}

    fn GetBool(&self,Index:isize) -> bool {
        let RetVal = self.Get(Index);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<bool>() {
                return *fdata;
            }else{
                return false;
            }
        }else{
            return false;
        }
    }

    fn GetInt(&self,Index:isize) -> i32 {
        let RetVal = self.Get(Index);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToInt(vv,true) as i32;
        }else{
            return 0;
        }
    }

    fn GetInt64(&self,Index:isize) -> i64 {
        let RetVal = self.Get(Index);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToInt64(vv,true) as i64;
        }else{
            return 0;
        }
    }

    fn GetString(&self,Index:isize) -> String {
        let RetVal = self.Get(Index);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<String>() {
                return fdata.clone();
            }else{
                return "".to_owned();
            }
        }else{
            return "".to_owned();
        }
    }

    fn GetDouble(&self,Index:isize) -> f64 {
        let RetVal = self.Get(Index);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToFloat(vv) as f64;
        }else{
            return 0.0;
        }
    }

    fn GetObject(&self,Index:isize)->STAROBJECT {
        let RetVal = self.Get(Index);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STAROBJECT>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn GetParaPkg(&self,Index:isize) -> STARPARAPKG {
        let RetVal = self.Get(Index);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARPARAPKG>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn GetBinBuf(&self,Index:isize) -> STARBINBUF {
        let RetVal = self.Get(Index);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARBINBUF>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn Clear<'a>(&'a self) -> &'a STARPARAPKG {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }   
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
	        return self;
        }
    }

    fn AppendFrom(&self,SrcParaPkg: &STARPARAPKG) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return false;
	        }       
            let SrcObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(SrcParaPkg) {    
                SrcObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if SrcObjData.ParaPkg == 0 as *mut c_void {
		        return false;
	        }                 
	        return FROMVS_BOOL(Star_SRPParaPkg_AppendFrom(ObjData.ParaPkg, SrcObjData.ParaPkg as *mut c_void));
        }
    }

    fn Set<'a>(&'a self,Index: isize, Value: &Any) -> &'a STARPARAPKG {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }           
        	let BasicSRPInterface = RustSRPGetBasicSRPInterface(ObjData.ServiceGroupID);
	        let SRPInterface = GetSRPServiceInterface(ObjData.ServiceGroupID, 0 as *mut c_void);
            let SubPara = [Value];
        	let NewParaPkg = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
	        if StarParaPkg_FromTuple_Sub(&SubPara, NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
                let c_s = CString::new("parameter not supported for starparapkg").unwrap();
                starrust_SRPI_ProcessError(
                        SRPInterface,
                        VSFAULT_WARNING,
                        CString::new("rust").unwrap().as_ptr(),
                        0,
                        c_s.as_ptr(),
                    );
		        Star_SRPParaPkg_Release(NewParaPkg);
		        return self;
	        }
	        let Number = Star_SRPParaPkg_GetNumber(ObjData.ParaPkg);
	        if Number == Index as i32 {
		        Star_SRPParaPkg_AppendFrom(ObjData.ParaPkg, NewParaPkg);
		        Star_SRPParaPkg_Release(NewParaPkg);
		        return self;
	        } else if Number < Index as i32 {
		        for i in Number..(Index as i32) {
			        Star_SRPParaPkg_InsertEmpty(ObjData.ParaPkg, i);
		        }
		        Star_SRPParaPkg_AppendFrom(ObjData.ParaPkg, NewParaPkg);
		        Star_SRPParaPkg_Release(NewParaPkg);
		        return self;
	        } else {
		        Star_SRPParaPkg_AppendFrom(ObjData.ParaPkg, NewParaPkg);
		        Star_SRPParaPkg_ExChange(ObjData.ParaPkg, Index as i32, Number);
		        Star_SRPParaPkg_Del(ObjData.ParaPkg, Number);
		        Star_SRPParaPkg_Release(NewParaPkg);
		        return self;
	        }
        }
    }

    fn Build<'a>(&'a self,tuple: &[&Any]) -> &'a STARPARAPKG {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
	        let BasicSRPInterface = RustSRPGetBasicSRPInterface(ObjData.ServiceGroupID);
	        let SRPInterface = GetSRPServiceInterface(ObjData.ServiceGroupID, 0 as *mut c_void);
	        if StarParaPkg_FromTuple_Sub(tuple, ObjData.ParaPkg as *mut c_void, BasicSRPInterface, 0, SRPInterface) == false {
                let c_s = CString::new("parameter not supported for starparapkg").unwrap();
                starrust_SRPI_ProcessError(
                        SRPInterface,
                        VSFAULT_WARNING,
                        CString::new("rust").unwrap().as_ptr(),
                        0,
                        c_s.as_ptr(),
                    );
        		return self;
	        }
	        return self;
        }
    }

    fn Free(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return;
	        }              
	        DeleteRustObjectAllRef(ObjData.RefItem);
	        return;
        }
    }

    fn Dispose(&self) {
	    self.Free();
    }

    fn ReleaseOwner(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let mut In_ObjData = STARGETRCMUT_STARPARAPKG!(self);
            if let Some(ObjData) = In_ObjData.as_mut() {   
                let mut fdata = &mut ObjData.ObjData; 
            	if fdata.ParaPkg == 0 as *mut c_void {
	    	        return;
	            }            
            	if fdata.IsClearedByStarCore == false {
	    	        if fdata.FreeFlag == true && fdata.ParaPkg != 0 as *mut c_void {
		    	        if StarRust_ModuleInitFlag == VS_TRUE {
			    	        Star_SRPParaPkg_ReleaseOwner(fdata.ParaPkg);
			            }
    		        }
	    	        fdata.IsClearedByStarCore = true;
	            }
	            if StarRust_ModuleInitFlag == VS_TRUE && StarRust_SRPControlInterface != 0 as *mut c_void && fdata.RefItem != 0 {
		            Star_SRPControl_UnRegScriptObject(StarRust_SRPControlInterface,fdata.RefItem,starrust_FreeScriptObject as *mut c_void, 0);
	            }
	            return;
            }else{
                return;
            }
        }    
    }

    fn AsDict<'a>(&'a self,IsDict: bool) -> &'a STARPARAPKG {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }           
        	Star_SRPParaPkg_AsDict(ObjData.ParaPkg, TOVS_BOOL(IsDict));
	        return self;
        }
    }

    fn IsDict(&self) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return false;
	        }            
        	return FROMVS_BOOL(Star_SRPParaPkg_IsDict(ObjData.ParaPkg));
        }
    }

    fn FromJSon(&self,Buf: &Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return false;
	        } 
	        let Local_Buf = SRPRustGetStr(Buf, true);
	        let ResultVal = Star_SRPParaPkg_FromJSon(ObjData.ParaPkg, Local_Buf);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Local_Buf as *mut c_void);
	        return FROMVS_BOOL(ResultVal);
        }
    }

    fn ToJSon(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return "".to_owned();
	        }         
        	let Buf = Star_SRPParaPkg_ToJSon(ObjData.ParaPkg);
	        if Buf == 0 as *mut c_char {
                return "".to_owned();
        	}
	        return SRPRustSetStr(Buf, true);
        }    
    }

    fn FromVecBool<'a>(&'a self,val:&Vec<bool>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertBool(ObjData.ParaPkg, i as i32,TOVS_BOOL(val[i]));
            }
            return self;
        }        
    }
    fn FromVecI8<'a>(&'a self,val:&Vec<i8>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertInt(ObjData.ParaPkg, i as i32,val[i] as i32);
            }
            return self;
        }        
    }
    fn FromVecU8<'a>(&'a self,val:&Vec<u8>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertInt(ObjData.ParaPkg, i as i32,val[i] as i32);
            }
            return self;
        }        
    }
    fn FromVecI16<'a>(&'a self,val:&Vec<i16>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
               Star_SRPParaPkg_InsertInt(ObjData.ParaPkg, i as i32,val[i] as i32);
            }
            return self;
        }        
    }
    fn FromVecU16<'a>(&'a self,val:&Vec<u16>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertInt(ObjData.ParaPkg, i as i32,val[i] as i32);
            }
            return self;
        }        
    }
    fn FromVecI32<'a>(&'a self,val:&Vec<i32>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertInt(ObjData.ParaPkg, i as i32,val[i] as i32);
            }
            return self;
        }        
    }
    fn FromVecU32<'a>(&'a self,val:&Vec<u32>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertInt(ObjData.ParaPkg, i as i32,val[i] as i32);
            }
            return self;
        }        
    }
    fn FromVecI64<'a>(&'a self,val:&Vec<i64>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertInt64(ObjData.ParaPkg, i as i32,val[i] as i64);
            }
            return self;
        }        
    }
    fn FromVecU64<'a>(&'a self,val:&Vec<u64>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertInt64(ObjData.ParaPkg, i as i32,val[i] as i64);
            }
            return self;
        }        
    }
    fn FromVecISize<'a>(&'a self,val:&Vec<isize>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                if size_of::<usize>() == 4 {
                    Star_SRPParaPkg_InsertInt(ObjData.ParaPkg, i as i32,val[i] as i32);                                                       
                }else{
                    Star_SRPParaPkg_InsertInt64(ObjData.ParaPkg, i as i32,val[i] as i64);
                }                   
            }
            return self;
        }        
    }
    fn FromVecUSize<'a>(&'a self,val:&Vec<usize>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                if size_of::<usize>() == 4 {
                    Star_SRPParaPkg_InsertInt(ObjData.ParaPkg, i as i32,val[i] as i32);                                                       
                }else{
                    Star_SRPParaPkg_InsertInt64(ObjData.ParaPkg, i as i32,val[i] as i64);
                }                  
            }
            return self;
        }        
    }
    fn FromVecF32<'a>(&'a self,val:&Vec<f32>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertFloat(ObjData.ParaPkg, i as i32,val[i] as f64);
            }
            return self;
        }        
    }
    fn FromVecF64<'a>(&'a self,val:&Vec<f64>) -> &'a STARPARAPKG{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                Star_SRPParaPkg_InsertFloat(ObjData.ParaPkg, i as i32,val[i] as f64);
            }
            return self;
        }        
    }

    fn FromVecString<'a>(&'a self,val:&Vec<String>) -> &'a STARPARAPKG
    {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                let (slen,cstr) = SRPRustGetStrEx(&val[i], false);
                Star_SRPParaPkg_InsertStrEx(ObjData.ParaPkg, i as i32, cstr,slen as u32);
                STARRUST_SAFERELEASESTR(&val[i], cstr); 
            }
            return self;
        }        
    } 

    fn FromVecStr<'a>(&'a self,val:&Vec<&'static str>) -> &'a STARPARAPKG
    {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return self;
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return self;
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return self;
	        }         
        	Star_SRPParaPkg_Clear(ObjData.ParaPkg);
            for i in 0..val.len() {
                let (slen,cstr) = SRPRustGetStrEx(&val[i], false);
                Star_SRPParaPkg_InsertStrEx(ObjData.ParaPkg, i as i32, cstr,slen as u32);
                STARRUST_SAFERELEASESTR(&val[i], cstr); 
            }
            return self;
        }        
    }

    fn ToVecBool(&self) -> Vec<bool>{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return Vec::new();
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return Vec::new();
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return Vec::new();
	        }       
            let num = Star_SRPParaPkg_GetNumber(ObjData.ParaPkg);
            if num  == 0 {
                return Vec::new();
            }

            let mut res : Vec<bool> = Vec::with_capacity(num as usize);
            res.resize(num as usize,false);
            for i in 0..num {
                res[i as usize] = FROMVS_BOOL(Star_SRPParaPkg_GetBool(ObjData.ParaPkg, i as i32));
            }
            return res;
        }        
    }

    fn ToVecInt(&self) -> Vec<i32>{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return Vec::new();
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return Vec::new();
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return Vec::new();
	        }       
            let num = Star_SRPParaPkg_GetNumber(ObjData.ParaPkg);
            if num  == 0 {
                return Vec::new();
            }

            let mut res : Vec<i32> = Vec::with_capacity(num as usize);
            res.resize(num as usize,0);
            for i in 0..num {
                res[i as usize] = Star_SRPParaPkg_GetInt(ObjData.ParaPkg, i as i32) as i32;
            }
            return res;
        }        
    }
    fn ToVecInt64(&self) -> Vec<i64>{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return Vec::new();
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return Vec::new();
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return Vec::new();
	        }       
            let num = Star_SRPParaPkg_GetNumber(ObjData.ParaPkg);
            if num  == 0 {
                return Vec::new();
            }

            let mut res : Vec<i64> = Vec::with_capacity(num as usize);
            res.resize(num as usize,0);
            for i in 0..num {
                res[i as usize] = Star_SRPParaPkg_GetInt64(ObjData.ParaPkg, i as i32) as i64;
            }
            return res;
        }        
    }
    fn ToVecString(&self) -> Vec<String>{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return Vec::new();
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return Vec::new();
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return Vec::new();
	        }       
            let num = Star_SRPParaPkg_GetNumber(ObjData.ParaPkg);
            if num  == 0 {
                return Vec::new();
            }

            let mut res : Vec<String> = Vec::with_capacity(num as usize);
            res.resize(num as usize,String::default());
            for i in 0..num {
   				let Str : *mut c_char;
    			let mut StrLen : u32 = 0;
	    		Str = Star_SRPParaPkg_GetStrEx(ObjData.ParaPkg, i as i32, &mut StrLen);
		    	let ttt = SRPRustSetStrEx(Str, StrLen as i32, true);                
                res[i as usize] = ttt;
            }
            return res;
        }        
    }
    fn ToVecDouble(&self) -> Vec<f64>{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return Vec::new();
	        }
            let ObjData : StarParaPkgBody;
            if let Some(fbody) = STARGETRCREF_STARPARAPKG!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return Vec::new();
            }          
        	if ObjData.ParaPkg == 0 as *mut c_void {
		        return Vec::new();
	        }       
            let num = Star_SRPParaPkg_GetNumber(ObjData.ParaPkg);
            if num  == 0 {
                return Vec::new();
            }

            let mut res : Vec<f64> = Vec::with_capacity(num as usize);
            res.resize(num as usize,0.0);
            for i in 0..num {
                res[i as usize] = Star_SRPParaPkg_GetFloat(ObjData.ParaPkg, i as i32) as f64;
            }
            return res;
        }        
    }  

}

/*----------------------------------------------------------------------------*/
/* StarBinBuf */
/*----------------------------------------------------------------------------*/

pub trait STARBINBUF_TRAIT {
    fn IsValid(&self) -> bool;
    fn ToString(&self) -> String;   

    fn GetOffset(&self) -> isize;
    fn Init(&self,BufSize: isize);
    fn Clear(&self);
    fn SaveToFile(&self,In_FileName: &Any, TxtFileFlag: bool) -> bool;
    fn LoadFromFile(&self,In_FileName: &Any, TxtFileFlag: bool) -> bool;
    fn Read( &self,Buf: &mut [u8], Offset: isize, Length: isize) -> isize;
    fn Write(&self,Offset: isize, Buf: &[u8], Length: isize) -> isize;
    fn Free(&self);
    fn ReleaseOwner(&self);
    fn Dispose( &self);
}

impl STARBINBUF_TRAIT for STARBINBUF {
    fn IsValid(&self) -> bool {
        if let Some(ObjData) = STARGETRCREF_STARBINBUF!(self) {
            return ObjData.IsValid;
        } else {
            return false;
        }
    }
    fn ToString(&self) -> String {
        let st: String;

        if let Some(ObjData) = STARGETRCREF_STARBINBUF!(self) {
            st = format!("{}", ObjData);
        } else {
            st = "binbuf object is invalid".to_owned();
        }
        return st;
    }       

    fn GetOffset(&self) -> isize {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarBinBufBody;
            if let Some(fbody) = STARGETRCREF_STARBINBUF!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.BinBuf == 0 as *mut c_void {
		        return 0;
	        }        
	        return Star_SRPBinBuf_GetOffset(ObjData.BinBuf) as usize as isize;
        }
    }

    fn Init(&self,BufSize: isize) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarBinBufBody;
            if let Some(fbody) = STARGETRCREF_STARBINBUF!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.BinBuf == 0 as *mut c_void {
		        return;
	        }         
        	Star_SRPBinBuf_Init(ObjData.BinBuf,BufSize as u32);
	        return
        }
    }

    fn Clear(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarBinBufBody;
            if let Some(fbody) = STARGETRCREF_STARBINBUF!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.BinBuf == 0 as *mut c_void {
		        return;
	        }           
        	Star_SRPBinBuf_Clear(ObjData.BinBuf);
	        return;
        }    
    }

    fn SaveToFile(&self,In_FileName: &Any, TxtFileFlag: bool) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarBinBufBody;
            if let Some(fbody) = STARGETRCREF_STARBINBUF!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.BinBuf == 0 as *mut c_void {
		        return false;
	        }          
        	let Buf = Star_SRPBinBuf_GetBuf(ObjData.BinBuf);
	        let Length = Star_SRPBinBuf_GetOffset(ObjData.BinBuf) as i32;
	        if Buf == 0 as *mut i8 || Length == 0 {
		        return false;
	        }
	        let FileName = SRPRustGetStr(In_FileName, true);
            let hFile : *const c_void;
	        if TxtFileFlag == true {
		        hFile = vs_file_fopen(FileName, CString::new("wt").unwrap().as_ptr());
	        } else {
		        hFile = vs_file_fopen(FileName, CString::new("wb").unwrap().as_ptr());
	        }
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, FileName as *mut c_void);
	        if hFile == 0 as *const c_void {
		        return false;
	        }
	        if Length != 0 {
		        starrust_fwrite(Buf as *const c_void, 1, Length as usize, hFile);
	        }
	        starrust_fclose(hFile);
	        return true;
        }
    }

    fn LoadFromFile(&self,In_FileName: &Any, TxtFileFlag: bool) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarBinBufBody;
            if let Some(fbody) = STARGETRCREF_STARBINBUF!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.BinBuf == 0 as *mut c_void {
		        return false;
	        }          
	        let FileName = SRPRustGetStr(In_FileName, true);
            let hFile : *const c_void;
	        if TxtFileFlag == true {
		        hFile = vs_file_fopen(FileName, CString::new("rt").unwrap().as_ptr());
	        } else {
		        hFile = vs_file_fopen(FileName, CString::new("rb").unwrap().as_ptr());
	        }
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, FileName as *mut c_void);
	        if hFile == 0 as *mut c_void {
		        return false;
	        }
	        starrust_fseek(hFile, 0, 2/*SEEK_END*/);
	        let mut Length = starrust_ftell(hFile);
	        starrust_fseek(hFile, 0, 0/*SEEK_SET*/);
            let mut Buf : Vec<u8> = Vec::with_capacity(Length as usize);
            Buf.resize(Length as usize,0);
        	Length = starrust_fread(Buf.as_mut_ptr() as *mut c_void, 1, Length as usize, hFile) as i32;
	        starrust_fclose(hFile);
	        Star_SRPBinBuf_Clear(ObjData.BinBuf);
	        let ResultVal = Star_SRPBinBuf_Set(ObjData.BinBuf, 0, Length as u32, Buf.as_mut_ptr() as *mut i8);
        	return FROMVS_BOOL(ResultVal);
        }
    }

    fn Read( &self,Buf: &mut [u8], Offset: isize, Length: isize) -> isize {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarBinBufBody;
            if let Some(fbody) = STARGETRCREF_STARBINBUF!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.BinBuf == 0 as *mut c_void {
		        return 0;
	        }           
	        let BufPtr = Star_SRPBinBuf_GetBufPtr(ObjData.BinBuf, 0);
	        let Size = Star_SRPBinBuf_GetOffset(ObjData.BinBuf) as u32;
	        if BufPtr == 0 as *mut i8 || Offset as i32 >= Size as i32 {
		        return 0;
	        }
            let mut r_Length = Length;
	        if Offset+Length > Size as isize {
		        r_Length = Size as isize - Offset;
	        }
	        vs_memcpy(Buf.as_mut_ptr() as *mut c_void, (BufPtr as usize + Offset as usize) as *const c_void, r_Length as isize);
	        return r_Length;
        }
    }

    fn Write(&self,Offset: isize, Buf: &[u8], Length: isize) -> isize {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarBinBufBody;
            if let Some(fbody) = STARGETRCREF_STARBINBUF!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.BinBuf == 0 as *mut c_void {
		        return 0;
	        }           
	        Star_SRPBinBuf_FromRaw(ObjData.BinBuf, VS_TRUE);
	        if Length == 0 {
		        return 0;
	        }
	        if Star_SRPBinBuf_Set(ObjData.BinBuf, Offset as u32, Length as u32, Buf.as_ptr() as *const i8 as *mut i8) == VS_FALSE {
		        return 0;
	        }
	        return Length;
        }
    }

    fn Free(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarBinBufBody;
            if let Some(fbody) = STARGETRCREF_STARBINBUF!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.BinBuf == 0 as *mut c_void {
		        return;
	        }          
        	DeleteRustObjectAllRef(ObjData.RefItem);
        	return;
        }
    }

    fn ReleaseOwner(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let mut In_ObjData = STARGETRCMUT_STARBINBUF!(self);
            if let Some(ObjData) = In_ObjData.as_mut() {   
                let mut fdata = &mut ObjData.ObjData; 
            	if fdata.BinBuf == 0 as *mut c_void {
	    	        return;
	            } 
            	if fdata.IsClearedByStarCore == false {
		            if fdata.FreeFlag == true && fdata.BinBuf != 0 as *mut c_void {
			            if StarRust_ModuleInitFlag == VS_TRUE {
				            Star_SRPBinBuf_ReleaseOwner(fdata.BinBuf);
			            }
		            }
		            fdata.IsClearedByStarCore = true;
	            }
	            if StarRust_ModuleInitFlag == VS_TRUE && StarRust_SRPControlInterface != 0 as *mut c_void && fdata.RefItem != 0 {
		            Star_SRPControl_UnRegScriptObject(StarRust_SRPControlInterface, fdata.RefItem, starrust_FreeScriptObject as *mut c_void, 0);
	            }
	            return;
            }else{
                return;
            }
        }
    }

    fn Dispose( &self) {
	    self.Free();
    }    
}

/*----------------------------------------------------------------------------*/
/* StarSXml */
/*----------------------------------------------------------------------------*/
pub trait STARSXML_TRAIT {
    fn IsValid(&self) -> bool;
    fn ToString(&self) -> String;

    fn LoadFromFile(&self,In_FileName: &Any) -> (bool,String);
    fn LoadFromBuf(&self,BinBuf: &STARBINBUF) -> (bool,String);
    fn LoadFromBufEx(&self,UtfArg: &Any) -> (bool,String);
    fn SaveToFile(&self,In_FileName: &Any) -> bool;
    fn SaveToBuf(&self,BinBuf:&STARBINBUF) -> bool;  
    fn GetStandalone(&self) -> String;
    fn GetVersion(&self) -> String;
    fn GetEncoding(&self) -> String;    
    fn FindElement(&self,Value: &Any) -> i64;
    fn FindElementEx(&self,arentElement: i64, Value: &Any) -> i64;
    fn FirstElement(&self,ParentElement: i64) -> i64;
    fn NextElement(&self,Element: i64) -> i64;
    fn ParentElement(&self,Element: i64) -> i64;
    fn GetElement(&self,Element: i64) -> String;
    fn GetElementEx(&self,Element: i64) -> String; 
    fn GetNs(&self,Element: i64)-> (bool,String,String);
    fn GetNsValue(&self,Element: i64, nsName: &Any) -> String;
    fn FindAttribute(&self,Element: i64, In_Name: &Any) -> i64;
    fn FirstAttribute(&self,Element: i64) -> i64;
    fn NextAttribute(&self,Attribute: i64) -> i64;
    fn GetAttributeName(&self,Attribute: i64) -> String;
    fn GetAttributeValue(&self,Attribute: i64) -> String;
    fn GetSingleText(&self,Element: i64) -> String;
    fn FirstText(&self,Element: i64) -> i64;
    fn NextText(&self,Text: i64) -> i64;
    fn GetText(&self,Text: i64)-> String;    
    fn SetDeclaration(&self,In_Version: &Any, In_Encoding: &Any, In_Standalone: &Any);
    fn RemoveDeclaration(&self);
    fn InsertElementBefore(&self,ParentElement: i64, Element: i64, In_Value: &Any) -> i64;
    fn InsertElementAfter(&self,ParentElement: i64, Element: i64, In_Value: &Any) -> i64;
    fn RemoveElement(&self,Element: i64);
    fn SetElement(&self,Element: i64, In_Value: &Any);
    fn SetNs(&self,Element: i64, nsName:&Any, nsValue: &Any);
    fn InsertTextBefore(&self,ParentElement: i64, Text: i64, In_Value: &Any, CDataFlag: bool) ->i64;
    fn InsertTextAfter(&self,ParentElement: i64, Text: i64, In_Value: &Any, CDataFlag: bool) -> i64;
    fn RemoveText(&self,Text: i64);
    fn SetText(&self,Text: i64, In_Value: &Any, CDataFlag: bool);
    fn InsertCommentBefore(&self,ParentElement: i64, Comment: i64, In_Value: &Any) -> i64;
    fn InsertCommentAfter(&self,ParentElement: i64, Comment: i64, In_Value: &Any) -> i64;
    fn  RemoveComment(&self,Comment: i64);
    fn SetComment(&self,Comment: i64, In_Value: &Any);
    fn SetAttribute(&self,Element: i64, In_Name: &Any, In_Value: &Any);
    fn RemoveAttribute(&self,Element: i64, In_Name: &Any);
    fn CopyElementBefore(&self,ParentElement: i64, Element: i64, SrcElement: i64) -> i64;
    fn CopyElementAfter(&self,ParentElement: i64, Element: i64, SrcElement: i64) -> i64;
    fn CopyChild(&self,DesElement: i64, SrcElement: i64) -> bool;
    fn Dup(&self,SrcSXML: &STARSXML) -> bool;
    fn Free(&self);
    fn ReleaseOwner(&self);
    fn Dispose(&self);    
}

impl STARSXML_TRAIT for STARSXML {
    fn IsValid(&self) -> bool {
        if let Some(ObjData) = STARGETRCREF_STARSXML!(self) {
            return ObjData.IsValid;
        } else {
            return false;
        }
    }
    fn ToString(&self) -> String {
        let st: String;

        if let Some(ObjData) = STARGETRCREF_STARSXML!(self) {
            st = format!("{}", ObjData);
        } else {
            st = "starsxml object is invalid".to_owned();
        }
        return st;
    }   

    fn LoadFromFile(&self,In_FileName: &Any) -> (bool,String) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return (false,"".to_owned());
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return (false,"".to_owned());
	        } 
        	let FileName = SRPRustGetStr(In_FileName, true);

            let mut ErrorInfo : Vec<*mut c_char> = Vec::new();
            ErrorInfo.push(0 as *mut c_char);

	        let ResultVal = Star_SRPSXML_LoadFromFile(ObjData.SXml, FileName, ErrorInfo.as_mut_ptr() as *mut [*mut c_char;1]);
            Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface,FileName as *mut c_void);
        	if ResultVal == VS_TRUE {
                return (true,"".to_owned());
        	} else {
		        if ErrorInfo[0] != 0 as *mut c_char {
                    return (false,SRPRustSetStr(ErrorInfo[0], true));
        		} else {
                    return (false,"".to_owned());
                }
            }
        }
    }

    fn LoadFromBuf(&self,BinBuf: &STARBINBUF) -> (bool,String) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return (false,"".to_owned());
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }         
        	let LocalBinBuf = RustObjectToBinBuf(BinBuf);
	        if LocalBinBuf == 0 as *mut c_void {
                return (false,"".to_owned());
        	}
            let mut ErrorInfo : Vec<*mut c_char> = Vec::new();
            ErrorInfo.push(0 as *mut c_char);            
	        let ResultVal = Star_SRPSXML_LoadFromBuf(ObjData.SXml,Star_SRPBinBuf_GetBuf(LocalBinBuf), ErrorInfo.as_mut_ptr() as *mut [*mut c_char;1]);
        	if ResultVal == VS_TRUE {
                return (true,"".to_owned());
        	} else {
		        if ErrorInfo[0] != 0 as *mut c_char {
                    return (false,SRPRustSetStr(ErrorInfo[0], true));
        		} else {
                    return (false,"".to_owned());
                }
            }
        }
    }

    fn LoadFromBufEx(&self,UtfArg: &Any) -> (bool,String) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return (false,"".to_owned());
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return (false,"".to_owned());
	        }           
        	let Info = SRPRustGetStr(UtfArg, false);
            let mut ErrorInfo : Vec<*mut c_char> = Vec::new();
            ErrorInfo.push(0 as *mut c_char);               
	        let ResultVal = Star_SRPSXML_LoadFromBuf(ObjData.SXml, Info as *const i8,ErrorInfo.as_mut_ptr() as *mut [*mut c_char;1]);
            STARRUST_SAFERELEASESTR(UtfArg, Info);
        	if ResultVal == VS_TRUE {
                return (true,"".to_owned());
        	} else {
		        if ErrorInfo[0] != 0 as *mut c_char {
                    return (false,SRPRustSetStr(ErrorInfo[0], true));
        		} else {
                    return (false,"".to_owned());
                }
            }            
        }
    }

    fn SaveToFile(&self,In_FileName: &Any) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return false;
	        } 
        	let FileName = SRPRustGetStr(In_FileName, true);
	        let ResultVal = Star_SRPSXML_SaveToFile(ObjData.SXml, FileName);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, FileName as *mut c_void);
	        return FROMVS_BOOL(ResultVal);
        }
    }

    fn SaveToBuf(&self,BinBuf:&STARBINBUF) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return false;
	        } 
	        let LocalBinBuf = RustObjectToBinBuf(BinBuf);
	        if LocalBinBuf == 0 as *mut c_void {
		        return false;
	        }
	        let ResultVal = Star_SRPSXML_SaveToBuf(ObjData.SXml, LocalBinBuf);
	        return FROMVS_BOOL(ResultVal);
        }      
    }

    fn GetStandalone(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        } 
        	let CharPtr = Star_SRPSXML_GetStandalone(ObjData.SXml);
	        if CharPtr !=0 as *mut c_char {
		        return SRPRustSetStr(CharPtr, false);
	        } else {
		        return "".to_owned();
	        }
        }
    }

    fn GetVersion(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        }         
	        let CharPtr = Star_SRPSXML_GetVersion(ObjData.SXml);
	        if CharPtr !=0 as *mut c_char {
		        return SRPRustSetStr(CharPtr, false);
	        } else {
		        return "".to_owned();
	        }
        }
    }

    fn GetEncoding(&self) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        } 
	        let CharPtr = Star_SRPSXML_GetEncoding(ObjData.SXml);
	        if CharPtr !=0 as *mut c_char {
		        return SRPRustSetStr(CharPtr, false);
	        } else {
		        return "".to_owned();
	        }
        }
    }

    fn FindElement(&self,Value: &Any) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        } 
	        let Name = SRPRustGetStr(Value, true);
	        let RetValue = Star_SRPSXML_FindElement(ObjData.SXml, Name);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Name as *mut c_void);
	        if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue) as i64;
	        }
        }
    }

    fn FindElementEx(&self,ParentElement: i64, Value: &Any) -> i64 {
       unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }         
	        let Name = SRPRustGetStr(Value, true);
	        let RetValue = Star_SRPSXML_FindElementEx(ObjData.SXml, starrust_ToPointer64(ParentElement), Name);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Name as *mut c_void);
	        if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue) as i64;
	        }
        }
    }

    fn FirstElement(&self,ParentElement: i64) -> i64 {
       unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }         
	        let RetValue = Star_SRPSXML_FirstElement(ObjData.SXml, starrust_ToPointer64(ParentElement));
	        if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue) as i64;
	        }
       }
    }

    fn NextElement(&self,Element: i64) -> i64 {
       unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }          
	        let RetValue = Star_SRPSXML_NextElement(ObjData.SXml, starrust_ToPointer64(Element));
	        if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue) as i64;
	        }
        }
    }

    fn ParentElement(&self,Element: i64) -> i64 {
       unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }         
	        let RetValue = Star_SRPSXML_ParentElement(ObjData.SXml, starrust_ToPointer64(Element));
	        if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue) as i64;
	        }
       }
    }

    fn GetElement(&self,Element: i64) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        }         
        	let Value = Star_SRPSXML_GetElement(ObjData.SXml, starrust_ToPointer64(Element));
	        if Value !=0 as *mut c_char {
		        return SRPRustSetStr(Value, false);
	        } else {
		        return "".to_owned();
	        }            
        }
    }

    fn GetElementEx(&self,Element: i64) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        }          
        	let mut Buf : [c_char;512] = mem::zeroed();
	        Star_SRPSXML_GetElementEx(ObjData.SXml, starrust_ToPointer64(Element), Buf.as_mut_ptr(), 512);
	        return SRPRustSetStr(Buf.as_mut_ptr(), true);
        }    
    }


    fn GetNs(&self,Element: i64)-> (bool,String,String) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return (false,"".to_owned(),"".to_owned());
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return (false,"".to_owned(),"".to_owned());
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return (false,"".to_owned(),"".to_owned());
	        } 
	        let mut Name: [c_char;128] = mem::zeroed();
            let mut Value : Vec<*mut c_char> = Vec::new();
            Value.push(0 as *mut c_char);              
        	if Star_SRPSXML_GetNs(ObjData.SXml, starrust_ToPointer64(Element), Name.as_mut_ptr(), 128, Value.as_mut_ptr() as *mut [*mut c_char;1]) == VS_FALSE {
                return (false,"".to_owned(),"".to_owned());
        	}
	        if Value[0] == 0 as *mut c_char {
                return (true,SRPRustSetStr(Name.as_mut_ptr(), true),"".to_owned());
        	} else {
                return (true,SRPRustSetStr(Name.as_mut_ptr(), true),SRPRustSetStr(Value[0], true));
        	}
        }
    }

    fn GetNsValue(&self,Element: i64, nsName: &Any) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        } 
        	let Name = SRPRustGetStr(nsName, true);
	        let Value = Star_SRPSXML_GetNsValue(ObjData.SXml, starrust_ToPointer64(Element), Name);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface,Name as *mut c_void);
	        if Value == 0 as *mut c_char {
		        return "".to_owned();
	        } else {
		        return SRPRustSetStr(Value, true);
	        }
        }
    }

    fn FindAttribute(&self,Element: i64, In_Name: &Any) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        } 
        	let Name = SRPRustGetStr(In_Name, true);
	        let RetValue = Star_SRPSXML_FindAttribute(ObjData.SXml, starrust_ToPointer64(Element), Name);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface,Name as *mut c_void);
        	if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn FirstAttribute(&self,Element: i64) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }         
        	let RetValue = Star_SRPSXML_FirstAttribute(ObjData.SXml, starrust_ToPointer64(Element));
        	if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue);
	        }            
        }
    }

    fn NextAttribute(&self,Attribute: i64) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }         
        	let RetValue = Star_SRPSXML_NextAttribute(ObjData.SXml, starrust_ToPointer64(Attribute));
        	if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue);
	        } 
        }
    }

    fn GetAttributeName(&self,Attribute: i64) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        }         
	        let Value = Star_SRPSXML_GetAttributeName(ObjData.SXml, starrust_ToPointer64(Attribute));
	        if Value == 0 as *mut c_char {
		        return "".to_owned();
	        } else {
		        return SRPRustSetStr(Value, true);
	        }
        }
    }

    fn GetAttributeValue(&self,Attribute: i64) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        }          
	        let Value = Star_SRPSXML_GetAttributeValue(ObjData.SXml,starrust_ToPointer64(Attribute));
	        if Value == 0 as *mut c_char {
		        return "".to_owned();
	        } else {
		        return SRPRustSetStr(Value, true);
	        }            
        }
    }

    fn GetSingleText(&self,Element: i64) -> String {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        }         
	        let Value = Star_SRPSXML_GetSingleText(ObjData.SXml, starrust_ToPointer64(Element));
	        if Value == 0 as *mut c_char {
		        return "".to_owned();
	        } else {
		        return SRPRustSetStr(Value, true);
	        } 
        }
    }

    fn FirstText(&self,Element: i64) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        } 
	        let RetValue = Star_SRPSXML_FirstText(ObjData.SXml, starrust_ToPointer64(Element));
        	if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue);
	        } 
        }
    }

    fn NextText(&self,Text: i64) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }         
	        let RetValue = Star_SRPSXML_NextText(ObjData.SXml, starrust_ToPointer64(Text));
        	if RetValue == 0 as *mut c_void {
		        return 0;
	        } else {
		        return starrust_PointerToInt64(RetValue);
	        } 
        }
    }

    fn GetText(&self,Text: i64)-> String{
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return "".to_owned();
	        }         
	        let Value = Star_SRPSXML_GetText(ObjData.SXml, starrust_ToPointer64(Text));
	        if Value == 0 as *mut c_char {
		        return "".to_owned();
	        } else {
		        return SRPRustSetStr(Value, true);
	        } 
        }
    }

    fn SetDeclaration(&self,In_Version: &Any, In_Encoding: &Any, In_Standalone: &Any) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }           
	        let Version = SRPRustGetStr(In_Version, false);
	        let Encoding = SRPRustGetStr(In_Encoding, false);
	        let Standalone = SRPRustGetStr(In_Standalone, false);
	        Star_SRPSXML_SetDeclaration(ObjData.SXml, Version, Encoding, Standalone);
	        STARRUST_SAFERELEASESTR(In_Version, Version);
	        STARRUST_SAFERELEASESTR(In_Encoding, Encoding);
	        STARRUST_SAFERELEASESTR(In_Standalone, Standalone);
	        return
        }
    }

    fn RemoveDeclaration(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }          
        	Star_SRPSXML_RemoveDeclaration(ObjData.SXml);
	        return
        }
    }

    fn InsertElementBefore(&self,ParentElement: i64, Element: i64, In_Value: &Any) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        } 
	        let InPtr = starrust_ToPointer64(ParentElement);
	        let InPtr2 = starrust_ToPointer64(Element);
	        let Value = SRPRustGetStr(In_Value, true);
	        let RetValue = Star_SRPSXML_InsertElementBefore(ObjData.SXml, InPtr, InPtr2, Value);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        if RetValue == 0 as *mut c_void {
		        return 0;
        	} else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn InsertElementAfter(&self,ParentElement: i64, Element: i64, In_Value: &Any) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }         
	        let InPtr = starrust_ToPointer64(ParentElement);
	        let InPtr2 = starrust_ToPointer64(Element);
	        let Value = SRPRustGetStr(In_Value, true);
	        let RetValue = Star_SRPSXML_InsertElementAfter(ObjData.SXml, InPtr, InPtr2, Value);
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        if RetValue == 0 as *mut c_void {
		        return 0;
        	} else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn RemoveElement(&self,Element: i64) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }           
        	Star_SRPSXML_RemoveElement(ObjData.SXml, starrust_ToPointer64(Element));
	        return;
        }
    }

    fn SetElement(&self,Element: i64, In_Value: &Any) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }   
	        let Value = SRPRustGetStr(In_Value, true);
	        Star_SRPSXML_SetElement(ObjData.SXml, starrust_ToPointer64(Element), Value);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        return;
        }
    }

    fn SetNs(&self,Element: i64, nsName:&Any, nsValue: &Any) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }  
 	        let Name = SRPRustGetStr(nsName, true);
	        let Value = SRPRustGetStr(nsValue, true);
	        Star_SRPSXML_SetNs(ObjData.SXml, starrust_ToPointer64(Element), Name, Value);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Name as *mut c_void);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        return;
        }
    }

    fn InsertTextBefore(&self,ParentElement: i64, Text: i64, In_Value: &Any, CDataFlag: bool) ->i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }         
	        let Value = SRPRustGetStr(In_Value, true);
	        let RetValue = Star_SRPSXML_InsertTextBefore(ObjData.SXml, starrust_ToPointer64(ParentElement), starrust_ToPointer64(Text), Value, TOVS_BOOL(CDataFlag));
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        if RetValue == 0 as *mut c_void {
		        return 0;
        	} else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn InsertTextAfter(&self,ParentElement: i64, Text: i64, In_Value: &Any, CDataFlag: bool) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }          
        	let Value = SRPRustGetStr(In_Value, true);
	        let RetValue = Star_SRPSXML_InsertTextAfter(ObjData.SXml, starrust_ToPointer64(ParentElement), starrust_ToPointer64(Text), Value, TOVS_BOOL(CDataFlag));
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        if RetValue == 0 as *mut c_void {
		        return 0;
        	} else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn RemoveText(&self,Text: i64) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }          
	        Star_SRPSXML_RemoveText(ObjData.SXml,starrust_ToPointer64(Text));
        	return;
        }
    }

    fn SetText(&self,Text: i64, In_Value: &Any, CDataFlag: bool) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }         
	        let Value = SRPRustGetStr(In_Value, true);
	        Star_SRPSXML_SetText(ObjData.SXml, starrust_ToPointer64(Text), Value, TOVS_BOOL(CDataFlag));
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        return;
        }
    }

    fn InsertCommentBefore(&self,ParentElement: i64, Comment: i64, In_Value: &Any) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }        
	        let Value = SRPRustGetStr(In_Value, true);
        	let RetValue = Star_SRPSXML_InsertCommentBefore(ObjData.SXml, starrust_ToPointer64(ParentElement), starrust_ToPointer64(Comment), Value);
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        if RetValue == 0 as *mut c_void {
		        return 0;
        	} else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn InsertCommentAfter(&self,ParentElement: i64, Comment: i64, In_Value: &Any) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }             
	        let Value = SRPRustGetStr(In_Value, true);
	        let RetValue = Star_SRPSXML_InsertCommentAfter(ObjData.SXml, starrust_ToPointer64(ParentElement), starrust_ToPointer64(Comment), Value);
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        if RetValue == 0 as *mut c_void {
		        return 0;
        	} else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn  RemoveComment(&self,Comment: i64) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }         
	        Star_SRPSXML_RemoveComment(ObjData.SXml, starrust_ToPointer64(Comment));
	        return;
        }
    }

    fn SetComment(&self,Comment: i64, In_Value: &Any) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }        
        	let Value = SRPRustGetStr(In_Value, true);
	        Star_SRPSXML_SetComment(ObjData.SXml, starrust_ToPointer64(Comment), Value);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        return;
        }
    }

    fn SetAttribute(&self,Element: i64, In_Name: &Any, In_Value: &Any) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }         
	        let Name = SRPRustGetStr(In_Name, true);
	        let Value = SRPRustGetStr(In_Value, true);
	        Star_SRPSXML_SetAttribute(ObjData.SXml, starrust_ToPointer64(Element), Name, Value);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Name as *mut c_void);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, Value as *mut c_void);
	        return;
        }
    }

    fn RemoveAttribute(&self,Element: i64, In_Name: &Any) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        }          
	        let Name = SRPRustGetStr(In_Name, true);
	        Star_SRPSXML_RemoveAttribute(ObjData.SXml, starrust_ToPointer64(Element), Name);
	        Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface,Name as *mut c_void);
	        return;
        }
    }

    fn CopyElementBefore(&self,ParentElement: i64, Element: i64, SrcElement: i64) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }           
        	let RetValue = Star_SRPSXML_CopyElementBefore(ObjData.SXml, starrust_ToPointer64(ParentElement), starrust_ToPointer64(Element),starrust_ToPointer64(SrcElement));
	        if RetValue == 0 as *mut c_void {
		        return 0;
        	} else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn CopyElementAfter(&self,ParentElement: i64, Element: i64, SrcElement: i64) -> i64 {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return 0;
	        }             
	        let RetValue = Star_SRPSXML_CopyElementAfter(ObjData.SXml, starrust_ToPointer64(ParentElement), starrust_ToPointer64(Element), starrust_ToPointer64(SrcElement));
	        if RetValue == 0 as *mut c_void {
		        return 0;
        	} else {
		        return starrust_PointerToInt64(RetValue);
	        }
        }
    }

    fn CopyChild(&self,DesElement: i64, SrcElement: i64) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return false;
	        }  
        	return FROMVS_BOOL(Star_SRPSXML_CopyChild(ObjData.SXml, starrust_ToPointer64(DesElement),starrust_ToPointer64(SrcElement)));
        }
    }

    fn Dup(&self,SrcSXML: &STARSXML) -> bool {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return false;
	        } 
            let ObjData1 : StarSXmlBody;
            if let Some(fbody1) = STARGETRCREF_STARSXML!(SrcSXML) {    
                ObjData1 = fbody1.ObjData.clone();
            }else{
                return false;
            }          
        	if ObjData1.SXml == 0 as *mut c_void {
		        return false;
	        }             
        	return FROMVS_BOOL(Star_SRPSXML_Copy(ObjData.SXml, ObjData1.SXml as *mut c_void));
        }
    }

    fn Free(&self) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarSXmlBody;
            if let Some(fbody) = STARGETRCREF_STARSXML!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
        	if ObjData.SXml == 0 as *mut c_void {
		        return;
	        } 
        	DeleteRustObjectAllRef(ObjData.RefItem);
        	return;
        }
    }

    fn ReleaseOwner(&self) {
       unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let mut In_ObjData = STARGETRCMUT_STARSXML!(self);
            if let Some(ObjData) = In_ObjData.as_mut() {   
                let mut fdata = &mut ObjData.ObjData; 
            	if fdata.IsClearedByStarCore == false {
		            if fdata.FreeFlag == true && fdata.SXml != 0 as *mut c_void {
			            if StarRust_ModuleInitFlag == VS_TRUE {
				            Star_SRPSXML_ReleaseOwner(fdata.SXml);
			            }
		            }
		            fdata.IsClearedByStarCore = true;
	            }
	            if StarRust_ModuleInitFlag == VS_TRUE && StarRust_SRPControlInterface != 0 as *mut c_void && fdata.RefItem != 0 {
		            Star_SRPControl_UnRegScriptObject(StarRust_SRPControlInterface, fdata.RefItem, starrust_FreeScriptObject as *mut c_void, 0);
	            }
	            return;
            }else{
                return;
            }
        }
    }

    fn Dispose(&self) {
	    self.Free();
    }    
}


/*----------------------------------------------------------------------------*/
/* StarService */
/*----------------------------------------------------------------------------*/
fn LuaTableToRust(ExpectedType: isize, SRPInterface: *const c_void, In_Index: i32) -> STARRESULT_TUPLE {
unsafe{    
	let mut RetValue : STARRESULT_TUPLE;
	let Val : STARRESULT;
	let mut IntTemp : i32;
    let mut Size : i32;
    let mut Index : i32;

    Index = In_Index;
	if In_Index < 0 {
		Index = In_Index - 1;
	}
	Size = 0;
	Star_SRPI_LuaPushNil(SRPInterface);
	loop {
		if Star_SRPI_LuaNext(SRPInterface, Index) == 0 {
			break;
		}
		match Star_SRPI_LuaType(SRPInterface, -2) {
    		VSLUATYPE_INT | VSLUATYPE_INT64 | VSLUATYPE_UWORD | VSLUATYPE_NUMBER =>
            {
		    	IntTemp = Star_SRPI_LuaToInt(SRPInterface, -2);
			    if IntTemp > 0 {
				    if IntTemp > Size {
					    Size = IntTemp;
    				}
	    		} else {
		    		Star_SRPI_LuaPop(SRPInterface, 2);
			    	return Vec::new();
    			}
            }
            _ =>
            {
	    		Star_SRPI_LuaPop(SRPInterface, 2);
		    	return Vec::new();
    		}
        }
		Star_SRPI_LuaPop(SRPInterface, 1);
	}
    RetValue = Vec::with_capacity(Size as usize);
    for ii in 0..Size as usize {
        RetValue.push(Some(Box::new(0)));
    }
	Star_SRPI_LuaPushNil(SRPInterface);
	loop {
		if Star_SRPI_LuaNext(SRPInterface, Index) == 0 {
			break;
		}
		match Star_SRPI_LuaType(SRPInterface, -2) {
    		VSLUATYPE_INT | VSLUATYPE_INT64 | VSLUATYPE_UWORD | VSLUATYPE_NUMBER =>
            {
		    	IntTemp = Star_SRPI_LuaToInt(SRPInterface, -2);
			    if IntTemp > 0 && IntTemp <= Size {
    				let mut ResultVal : bool = false;
	    			let Val = LuaToRustObject(ExpectedType, SRPInterface, -1, &mut ResultVal);
		    		if ResultVal == false {
			    		Star_SRPI_LuaPop(SRPInterface, 2);
				    	return Vec::new();
    				}
	    			RetValue[(IntTemp-1) as usize] = Val;
		    	} else {
			    	Star_SRPI_LuaPop(SRPInterface, 2);
				    return Vec::new();
    			}
            }
		    _ =>
            {
	    		Star_SRPI_LuaPop(SRPInterface, 2);
		    	return Vec::new();
		    }   
        }
		Star_SRPI_LuaPop(SRPInterface, 1);
	}
	return RetValue;
}    
}

fn RustToLuaTable(SRPInterface: *const c_void, arr_data: &[&Any], ToRaw: bool) {
unsafe{    
	let num_index : i32;
	let mut localobject : &Any;

	Star_SRPI_LuaNewTable(SRPInterface);
	for num_index in 0..arr_data.len() {
		Star_SRPI_LuaPushInt(SRPInterface, (num_index+1) as i32);
		localobject = arr_data[num_index];
		RustObjectToLua(SRPInterface, localobject, ToRaw);
		if Star_SRPI_LuaIsNil(SRPInterface, -1) == VS_TRUE {
			Star_SRPI_LuaPop(SRPInterface, 3);
			Star_SRPI_LuaPushNil(SRPInterface);
			return;
		}
		Star_SRPI_LuaSetTable(SRPInterface, -3);
	}
	return;
}    
}

fn RustObjectToLua(SRPInterface: *const c_void, Object: &Any, ToRaw: bool) -> bool {
unsafe{    
	if let Some(fdata) = Object.downcast_ref::<bool>() {
		Star_SRPI_LuaPushBool(SRPInterface, TOVS_BOOL(*fdata));
    }else if let Some(fdata) = Object.downcast_ref::<i8>() {
		Star_SRPI_LuaPushInt(SRPInterface, (*fdata) as i32);
    }else if let Some(fdata) = Object.downcast_ref::<i16>() {
		Star_SRPI_LuaPushInt(SRPInterface, (*fdata) as i32);
    }else if let Some(fdata) = Object.downcast_ref::<i32>() {
		Star_SRPI_LuaPushInt(SRPInterface, (*fdata) as i32);
    }else if let Some(fdata) = Object.downcast_ref::<i64>() {
		Star_SRPI_LuaPushInt64(SRPInterface, (*fdata) as i64);
    }else if let Some(fdata) = Object.downcast_ref::<isize>() {    
        if size_of::<isize>() == 4 {    
			Star_SRPI_LuaPushInt(SRPInterface, (*fdata) as i32);
        }else{
            Star_SRPI_LuaPushInt64(SRPInterface, (*fdata) as i64);
		}
    }else if let Some(fdata) = Object.downcast_ref::<u8>() {        
        Star_SRPI_LuaPushInt(SRPInterface, (*fdata) as u32 as i32);
    }else if let Some(fdata) = Object.downcast_ref::<u16>() {        
        Star_SRPI_LuaPushInt(SRPInterface, (*fdata) as u32 as i32);
    }else if let Some(fdata) = Object.downcast_ref::<u32>() {        
        Star_SRPI_LuaPushInt(SRPInterface, (*fdata) as u32 as i32);
    }else if let Some(fdata) = Object.downcast_ref::<u64>() {        
        Star_SRPI_LuaPushInt64(SRPInterface, (*fdata) as u64 as i64);
    }else if let Some(fdata) = Object.downcast_ref::<usize>() {        
        if size_of::<usize>() == 4 {    
			Star_SRPI_LuaPushInt(SRPInterface, (*fdata) as u32 as i32);
        }else{
            Star_SRPI_LuaPushInt64(SRPInterface, (*fdata) as u64 as i64);
		}
    }else if let Some(fdata) = Object.downcast_ref::<f32>() {        
        Star_SRPI_LuaPushNumber(SRPInterface, (*fdata) as f64);
    }else if let Some(fdata) = Object.downcast_ref::<f64>() {        
        Star_SRPI_LuaPushNumber(SRPInterface, (*fdata) as f64);
    }else if let Some(fdata) = Object.downcast_ref::<String>() {        
        let (slen,cstr) = SRPRustGetStrEx(fdata, false);
        Star_SRPI_LuaPushLString(SRPInterface, cstr,slen as u32);
        STARRUST_SAFERELEASESTR(fdata, cstr);
    }else if let Some(fdata) = Object.downcast_ref::<&str>() {        
        let (slen,cstr) = SRPRustGetStrEx(fdata, false);
        Star_SRPI_LuaPushLString(SRPInterface, cstr,slen as u32);
        STARRUST_SAFERELEASESTR(fdata, cstr);
    }else if let Some(fdata) = Object.downcast_ref::<STARPARAPKG>() {        
		let NewParaPkg = RustObjectToParaPkg(fdata);
		if NewParaPkg != 0 as *mut c_void {
			Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_FALSE);
		} else {
			Star_SRPI_LuaPushNil(SRPInterface);
			return true;
		}
    }else if let Some(fdata) = Object.downcast_ref::<STARBINBUF>() {        
		let SRPBinBufInterface = RustObjectToBinBuf(fdata);
		if SRPBinBufInterface != 0 as *mut c_void {
			Star_SRPI_LuaPushBinBuf(SRPInterface, SRPBinBufInterface, VS_FALSE);
		} else {
			Star_SRPI_LuaPushNil(SRPInterface);
			return true;
		}
    }else if let Some(fdata) = Object.downcast_ref::<STARSXML>() {            
		let SRPSXmlInterface = RustObjectToSXml(fdata);
		if SRPSXmlInterface != 0 as *mut c_void {
			Star_SRPI_LuaPushSXml(SRPInterface, SRPSXmlInterface, VS_FALSE);
		} else {
			Star_SRPI_LuaPushNil(SRPInterface);
			return true;
		}
    }else if let Some(fdata) = Object.downcast_ref::<STAROBJECT>() {            
		let SRPObject = RustObjectToSRPObject(fdata);
		Star_SRPI_LuaPushObject(SRPInterface, SRPObject);
    }else if let Some(fdata) = Object.downcast_ref::<HashMap<&str,&Any>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);
    }else if let Some(fdata) = Object.downcast_ref::<Vec<bool>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);      
    }else if let Some(fdata) = Object.downcast_ref::<Vec<i8>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);    
    }else if let Some(fdata) = Object.downcast_ref::<Vec<u8>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);         
    }else if let Some(fdata) = Object.downcast_ref::<Vec<i16>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);     
    }else if let Some(fdata) = Object.downcast_ref::<Vec<u16>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);   
    }else if let Some(fdata) = Object.downcast_ref::<Vec<i32>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);     
    }else if let Some(fdata) = Object.downcast_ref::<Vec<u32>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);      
    }else if let Some(fdata) = Object.downcast_ref::<Vec<isize>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);      
    }else if let Some(fdata) = Object.downcast_ref::<Vec<usize>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);     
    }else if let Some(fdata) = Object.downcast_ref::<Vec<i64>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);     
    }else if let Some(fdata) = Object.downcast_ref::<Vec<u64>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);         
    }else if let Some(fdata) = Object.downcast_ref::<Vec<f32>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);         
    }else if let Some(fdata) = Object.downcast_ref::<Vec<f64>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);     
    }else if let Some(fdata) = Object.downcast_ref::<Vec<String>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);    
    }else if let Some(fdata) = Object.downcast_ref::<Vec<&str>>() {   
        let BasicSRPInterface = Star_SRPI_GetBasicInterface(SRPInterface)                 ;
		let NewParaPkg = Star_SRPI_GetParaPkgInterface(SRPInterface);
   		if StarParaPkg_FromTuple_Sub(&[fdata], NewParaPkg, BasicSRPInterface, 0, SRPInterface) == false {
			Star_SRPParaPkg_Release(NewParaPkg);
			Star_SRPBasic_Release(BasicSRPInterface);
			Star_SRPI_LuaPushNil(SRPInterface);
		    return false;
		}
		Star_SRPBasic_Release(BasicSRPInterface);
		Star_SRPI_LuaPushParaPkg(SRPInterface, NewParaPkg, VS_TRUE);                                                                   
    }else{
		return false;
    }
	return true;
}    
}

/* -- ExpectedType should be zero, for rust does not support reflect --*/
fn LuaToRustObject(ExpectedType: isize, SRPInterface: *const c_void, Index: i32, ResultVal : &mut bool) -> STARRESULT {
unsafe{   
	(*ResultVal) = true;
	match Star_SRPI_LuaType(SRPInterface, Index) {
    	VSLUATYPE_INT => return Some(Box::new(Star_SRPI_LuaToInt(SRPInterface, Index))),
    	VSLUATYPE_INT64 => return Some(Box::new(Star_SRPI_LuaToInt64(SRPInterface, Index))),
	    VSLUATYPE_UWORD => return Some(Box::new(Star_SRPI_LuaToUWord(SRPInterface, Index) as usize)),
    	VSLUATYPE_NUMBER => return Some(Box::new(Star_SRPI_LuaToNumber(SRPInterface, Index) as f64)),
	    VSLUATYPE_BOOL => return Some(Box::new(FROMVS_BOOL(Star_SRPI_LuaToBool(SRPInterface, Index)))),
	    VSLUATYPE_STRING =>
		    {
				let StringBuf : *mut c_char;
				let mut StringSize : u32 = 0;

				StringBuf = Star_SRPI_LuaToLString(SRPInterface, Index, &mut StringSize);
				if StringBuf != 0 as *mut c_char {
					let rrr = SRPRustSetStrEx(StringBuf, StringSize as i32, true);
					if rrr.len() != 0 || StringSize == 0 {
						return Some(Box::new(rrr));
					}
					let mut Temp : Vec<u8> = Vec::with_capacity(StringSize as usize);
					if StringSize != 0 {
                        Temp.resize(StringSize as usize,0);
                        vs_memcpy(Temp.as_mut_ptr() as *mut c_void, StringBuf as *const c_void, StringSize as isize);
					}
					return Some(Box::new(Temp));
				} else {
					return Some(Box::new("".to_owned()));
				}
			},
		VSLUATYPE_TABLE =>
			if Star_SRPI_LuaTableToParaPkg(SRPInterface, Index, 0 as *mut c_void, VS_TRUE) == VS_TRUE {
				return Some(Box::new(LuaTableToRust(ExpectedType, SRPInterface, Index)));
			}else{
                return None;
            },
		VSLUATYPE_FUNCTION | VSLUATYPE_USERDATA | VSLUATYPE_LIGHTUSERDATA =>
			return Some(Box::new(ToStarObject(Star_SRPI_LuaToRaw(SRPInterface, Index, VS_FALSE), SRPInterface as *mut c_void, true))),
		VSLUATYPE_OBJECT =>
			return Some(Box::new(ToStarObject(Star_SRPI_LuaToObject(SRPInterface, Index), SRPInterface as *mut c_void, false))),
		VSLUATYPE_PARAPKG =>
			return Some(Box::new(ToStarParaPkg(Star_SRPI_LuaToParaPkg(SRPInterface, Index), Star_SRPI_GetServiceGroupID(SRPInterface), false))),
		VSLUATYPE_BINBUF =>
			{
				let BinBuf : *mut c_void = Star_SRPI_LuaToBinBuf(SRPInterface, Index);
				if Star_SRPBinBuf_IsFromRaw(BinBuf) == VS_FALSE {
					return Some(Box::new(ToStarBinBuf(Star_SRPI_LuaToBinBuf(SRPInterface, Index), Star_SRPI_GetServiceGroupID(SRPInterface), false)));
				} else {
					let StringBuf = Star_SRPBinBuf_GetBuf(BinBuf);
					let StringSize = Star_SRPBinBuf_GetOffset(BinBuf) as i32;
                    let mut Temp : Vec<u8> = Vec::with_capacity(StringSize as usize);
					if StringSize == 0 {
						return Some(Box::new(Temp));
					} else {
                        Temp.resize(StringSize as usize,0);
                        vs_memcpy(Temp.as_mut_ptr() as *mut c_void, StringBuf as *const c_void, StringSize as isize);
						return Some(Box::new(Temp));
					}
				}
			},
		VSLUATYPE_SXML =>
			return Some(Box::new(ToStarSXml(Star_SRPI_LuaToSXml(SRPInterface, Index),Star_SRPI_GetServiceGroupID(SRPInterface), false))),
		VSLUATYPE_NIL =>
			return None,
		_ =>
        {
			(*ResultVal) = false;
			return None;
        },
	}
}    
}

fn SRPObject_AttributeToRustObject(selfval: &StarObjectBody, AttributeIndex: u8, SRPInterface: *const c_void, AttributeType: u8, AttributeLength: i32, StructID: *const VS_UUID, BufOffset: u32, Buf: *mut u8, UseStructObject: bool) -> STARRESULT {
	let AtomicStructObject : *const c_void;
unsafe{
	match AttributeType {
	VSTYPE_BOOL => return Some(Box::new(FROMVS_BOOL(*((Buf as usize +BufOffset as usize) as *const VS_BOOL)))),
    VSTYPE_INT8 => return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const i8))),
    VSTYPE_UINT8 => return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const u8))),
    VSTYPE_INT16 =>	return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const i16))),
    VSTYPE_UINT16 =>return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const u16))),
	VSTYPE_INT32 => return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const i32))),
    VSTYPE_UINT32 => return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const u32))),
    VSTYPE_INT64 => return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const i64))),
    VSTYPE_FLOAT => return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const f32))),
    VSTYPE_DOUBLE =>return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const f64))),
    VSTYPE_LONG =>return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const i32))),
    VSTYPE_ULONG=>return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const u32))),
    VSTYPE_LONGHEX=>return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const i32))),
    VSTYPE_ULONGHEX=>return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const u32))),
    VSTYPE_VSTRING =>
        {
            let aaa : *mut VS_VSTRING = (Buf as usize +BufOffset as usize) as *mut VS_VSTRING;
    		if (*aaa).Buf == 0 as *mut c_char {
	    		return Some(Box::new("".to_owned()));
		    } else {
    			return Some(Box::new(SRPRustSetStr((*aaa).Buf, true)));
	    	}
        },
	VSTYPE_STRUCT => 
        {
    		let AtomicStructObject = Star_SRPI_GetAtomicObject(SRPInterface, StructID as *mut VS_UUID);
		    if AtomicStructObject == 0 as *mut c_void {
			    return None;
		    }
		    {
			    let mut NewObject : STARRESULT_TUPLE;
	    		let mut StructAttributeInfo: VS_ATTRIBUTEINFO = VS_ATTRIBUTEINFO::new();

		    	let AttributeNumber = Star_SRPI_GetAtomicStructAttributeNumber(SRPInterface, AtomicStructObject);
			    NewObject = Vec::with_capacity(AttributeNumber as usize);
			    for i in 0..AttributeNumber{
				    if Star_SRPI_GetAtomicStructAttributeInfoEx(SRPInterface, AtomicStructObject, i as u8, &mut StructAttributeInfo) == VS_TRUE {
					    NewObject.push(SRPObject_AttributeToRustObject(selfval, StructAttributeInfo.AttributeIndex, SRPInterface, StructAttributeInfo.Type, StructAttributeInfo.Length, &mut StructAttributeInfo.StructID, StructAttributeInfo.Offset as u32 + BufOffset, Buf, UseStructObject));
				    }
			    }
			    return Some(Box::new(NewObject));
		    }
        },

	VSTYPE_COLOR => return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const u32))),
	VSTYPE_CHAR =>
		if AttributeLength == 1 {
			return Some(Box::new(*((Buf as usize +BufOffset as usize) as *const i8)));
		} else {
			return Some(Box::new(SRPRustSetStr((Buf as usize +BufOffset as usize) as *mut c_char, true)));
		},
	VSTYPE_UUID | VSTYPE_STATICID =>
		return Some(Box::new(SRPRustSetStr(Star_SRPI_UuidToString(SRPInterface, (Buf as usize +BufOffset as usize) as *mut VS_UUID) as *mut c_char, false))),
	_ =>
		return None,
	}
}    
}

fn SRPObject_RustObjectToAttribute(SRPInterface: *const c_void, AttributeType: u8, AttributeLength: i32, StructID: *const VS_UUID, ObjectTemp: &Any, BufOffset: u32, Buf: *const u8) -> bool {
unsafe{
	match AttributeType {
	VSTYPE_BOOL =>
        {
            if let Some(fdata) = ObjectTemp.downcast_ref::<bool>() {
    			*((Buf as usize +BufOffset as usize) as *mut VS_BOOL) = TOVS_BOOL(*fdata);
		    } else {
                *((Buf as usize +BufOffset as usize) as *mut VS_BOOL) = VS_FALSE;
    		}
		    return true;
        },
	VSTYPE_INT8 =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut i8) = UIntValue as i8;
		        return true;
            }
        },
	VSTYPE_UINT8 =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut u8) = UIntValue as u8;
		        return true;
            }
        },
	VSTYPE_INT16 =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut i16) = UIntValue as i16;
		        return true;
            }
        },
	VSTYPE_UINT16 =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut u16) = UIntValue as u16;
		        return true;
            }
        },
	VSTYPE_INT32 =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut i32) = UIntValue as i32;
		        return true;
            }
        },
	VSTYPE_UINT32 =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut u32) = UIntValue as u32;
		        return true;
            }
        },
	VSTYPE_INT64 =>
        {
            *((Buf as usize +BufOffset as usize) as *mut i64) = SRPRustToLong(ObjectTemp, true);
	        return true;
        },
	VSTYPE_FLOAT =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == true {
                *((Buf as usize +BufOffset as usize) as *mut f32) = UIntValue as f32;
			    return true
		    }
            let (FloatValue,resf) = SRPRustGetFloat(ObjectTemp);
    		if resf == false {
	    		return false;
    		}
            *((Buf as usize +BufOffset as usize) as *mut f32) = FloatValue as f32;
    		return true;
        },
	VSTYPE_DOUBLE =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == true {
                *((Buf as usize +BufOffset as usize) as *mut f64) = UIntValue as f64;
			    return true
		    }
            let (FloatValue,resf) = SRPRustGetFloat(ObjectTemp);
    		if resf == false {
	    		return false;
    		}
            *((Buf as usize +BufOffset as usize) as *mut f64) = FloatValue as f64;
    		return true;
        },
	VSTYPE_LONG =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut i32) = UIntValue as i32;
		        return true;
            }
        },
	VSTYPE_ULONG =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut u32) = UIntValue as u32;
		        return true;
            }
        },    
	VSTYPE_LONGHEX =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut i32) = UIntValue as i32;
		        return true;
            }
        },    
	VSTYPE_ULONGHEX =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut u32) = UIntValue as u32;
		        return true;
            }
        },    
	VSTYPE_VSTRING =>
        if SRPRustIsString(ObjectTemp) {
			let CharValue = SRPRustGetStr(ObjectTemp, true);
			if CharValue == 0 as *mut c_char {
				return false;
			}
            let vs = (Buf as usize +BufOffset as usize) as *mut VS_VSTRING;
            Star_SRPI_ExpandVStringBufSize(SRPInterface,vs,(vs_string_strlen(CharValue)+1) as u32);
			vs_string_strcpy((*vs).Buf, CharValue);
			Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface,CharValue as *mut c_void);
			return true;
		} else {
			return false;
		},

	VSTYPE_STRUCT =>
		if let Some(fdata) = ObjectTemp.downcast_ref::<STARRESULT_TUPLE>() {
			let AtomicStruct = Star_SRPI_GetAtomicObject(SRPInterface, StructID as *mut VS_UUID);
			if AtomicStruct == 0 as *mut c_void {
				return false;
			}
            let mut AttributeInfo : VS_ATTRIBUTEINFO = VS_ATTRIBUTEINFO::new();
			for num_index in 0..fdata.len() {
				let i = num_index;
				if i < Star_SRPI_GetAtomicStructAttributeNumber(SRPInterface, AtomicStruct) as usize {
					Star_SRPI_GetAtomicStructAttributeInfoEx(SRPInterface, AtomicStruct,i as u8, &mut AttributeInfo);
					let LocalObjectTemp = &fdata[num_index];
					if SRPObject_RustObjectToAttribute(SRPInterface, AttributeInfo.Type, AttributeInfo.Length, 0 as *const VS_UUID, LocalObjectTemp, AttributeInfo.Offset as u32+BufOffset, Buf) == false {
						return false;
					}
				}
			}
    		return true;
        }else{
            return false;
        },

	VSTYPE_COLOR =>
        {
            let (UIntValue,res) =  SRPRustGetInt(ObjectTemp, false);
		    if res == false {
				return false;
			}else{
                *((Buf as usize +BufOffset as usize) as *mut u32) = UIntValue as u32;
		        return true;
            }
        },     

	VSTYPE_CHAR => 
		if SRPRustIsString(ObjectTemp) {
			let CharValue = SRPRustGetStr(ObjectTemp, true);
			if CharValue == 0 as *mut c_char {
				return false;
			}
			starrust_strncpy((Buf as usize +BufOffset as usize) as *mut c_char, CharValue, AttributeLength as isize);
			Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, CharValue as *mut c_void);
			return true;
		} else {
			return false;
		},

	VSTYPE_UUID | VSTYPE_STATICID => 
		if SRPRustIsString(ObjectTemp) {
			let CharValue = SRPRustGetStr(ObjectTemp, false);
			if CharValue == 0 as *mut c_char {
				return false;
			}
            Star_SRPI_StringToUuid(SRPInterface,CharValue as *const i8,(Buf as usize +BufOffset as usize) as *mut VS_UUID);
			STARRUST_SAFERELEASESTR(ObjectTemp, CharValue);
			return true;
		} else {
			return false;
		},
    _ =>
    	return false,
    }
}    
}

fn SRPObject_CreateNewObject(SRPInterface: *const c_void,SRPClassObject: *const c_void,Type: i32, ObjectID: *mut VS_UUID, ClientID: u32, QueueAttrName: *const c_char, ParentObject: *const c_void, ObjectNameString: *const c_char, InitString: *const c_char, InitBuf: *const c_void) -> *const c_void {
unsafe{    
	let mut AttributeInfo: VS_ATTRIBUTEINFO = VS_ATTRIBUTEINFO::new();
	let mut ObjectClassID: VS_UUID = VS_UUID::new();

	Star_SRPI_GetID(SRPInterface, SRPClassObject as *mut c_void, &mut ObjectClassID);
	if ParentObject != 0 as *const c_void {
		if QueueAttrName != 0 as *const c_char {
			if Star_SRPI_GetAttributeInfoEx(SRPInterface, Star_SRPI_GetClass(SRPInterface, ParentObject as *mut c_void), QueueAttrName, &mut AttributeInfo) == VS_FALSE {
                let S = format!("Get Parent Attribute [{}]", SRPRustSetStr(QueueAttrName as *mut c_char,false));
                RustPrintError(
                    VSFAULT_WARNING,
                    CString::new(S)
                    .unwrap()
                    .as_ptr(),
                );                 
				return 0 as *const c_void;
			}
		} else {
			if Star_SRPI_IsObject(SRPInterface, ParentObject as *mut c_void) == VS_TRUE {
				let AttributeNumber = Star_SRPI_GetAttributeNumber(SRPInterface, Star_SRPI_GetClass(SRPInterface, ParentObject as *mut c_void));
                let mut i : i32 = 0;
                loop{
                    if i >= AttributeNumber{
                        break;
                    }
					Star_SRPI_GetAttributeInfo(SRPInterface, Star_SRPI_GetClass(SRPInterface, ParentObject as *mut c_void), i as u8, &mut AttributeInfo);
					if AttributeInfo.Type == VSTYPE_PTR && AttributeInfo.SyncType == VS_TRUE && starrust_uuidisequal(&AttributeInfo.StructID, &ObjectClassID) == VS_TRUE || Star_SRPI_IsInst(SRPInterface, &mut AttributeInfo.StructID, SRPClassObject as *mut c_void) == VS_TRUE {
						break;
					}
                    i = i+1;
				}
				if i >= AttributeNumber {
                    i = 0;
					loop{
                        if i >= AttributeNumber {
                            break;
                        }
						Star_SRPI_GetAttributeInfo(SRPInterface, Star_SRPI_GetClass(SRPInterface, ParentObject as *mut c_void), i as u8, &mut AttributeInfo);
						if AttributeInfo.Type == VSTYPE_PTR && AttributeInfo.SyncType == VS_TRUE && starrust_uuidisvalid(&AttributeInfo.StructID) == VS_TRUE {
							break;
						}
                        i = i + 1;
					}
					if i >= AttributeNumber {
                        RustPrintError(
                            VSFAULT_WARNING,
                            CString::new("not Found Parent Sync Attribute Queue".to_owned())
                            .unwrap()
                            .as_ptr(),
                        );                         
						return 0 as *const c_void;
					}
				}
			} else {
				AttributeInfo.AttributeIndex = 0;
			}
		}
	}
	let NewSRPObject: *const c_void;
	if ParentObject != 0 as *const c_void {
		NewSRPObject = Star_SRPI_IMallocObjectEx(SRPInterface, ObjectID, ParentObject as *mut c_void, AttributeInfo.AttributeIndex, &mut ObjectClassID, InitBuf as *mut c_void)
	} else {
		NewSRPObject = Star_SRPI_IMallocObjectLEx(SRPInterface, ObjectID, &mut ObjectClassID, InitBuf as *mut c_void);
	}
	if NewSRPObject != 0 as *const c_void {
		Star_SRPI_SetSourceScript(SRPInterface, NewSRPObject as *mut c_void, StarRust_ScriptInterfaceIndex);
		if ObjectNameString != 0 as *const c_char &&  vs_string_strlen(ObjectNameString) != 0 {
			Star_SRPI_SetName(SRPInterface, NewSRPObject as *mut c_void, ObjectNameString);
		}
		if InitString != 0 as *const c_char && vs_string_strlen(InitString) != 0 {
			Star_SRPI_LuaInitObject(SRPInterface, NewSRPObject as *mut c_void, InitString);
		}
	}
	return NewSRPObject;
}    
}

//+++++++++++++++++++++++++++++++++++

static mut OBJECT_CREATE_ATTACH_BUF : *const c_char = 0 as *const c_char;

pub trait STAROBJECT_TRAIT {
    fn IsValid(&self) -> bool;
    fn ToString(&self) -> String;
    fn Get(&self,InVar: &Any) -> STARRESULT;
    fn GetBool(&self,InVar: &Any) -> bool;
    fn GetInt(&self,InVar: &Any) -> i32;
    fn GetInt64(&self,InVar: &Any) -> i64;
    fn GetString(&self,InVar: &Any) -> String;
    fn GetDouble(&self,InVar: &Any) -> f64;
    fn GetObject(&self,InVar: &Any)->STAROBJECT;
    fn GetParaPkg(&self,InVar: &Any) -> STARPARAPKG;
    fn GetBinBuf(&self,InVar: &Any) -> STARBINBUF;  
    fn Set(&self,Name: &Any, Para: &Any);
    fn Call(&self,FuncName: &Any, Args: &[&Any]) -> STARRESULT;
    fn CallBool(&self,FuncName: &Any, Args: &[&Any]) -> bool;
    fn CallInt(&self,FuncName: &Any, Args: &[&Any]) -> i32;
    fn CallInt64(&self,FuncName: &Any, Args: &[&Any]) -> i64;
    fn CallString(&self,FuncName: &Any, Args: &[&Any]) ->String;
    fn CallDouble(&self,FuncName: &Any, Args: &[&Any]) ->f64;
    fn CallObject(&self,FuncName: &Any, Args: &[&Any]) ->STAROBJECT;
    fn CallParaPkg(&self,FuncName: &Any, Args: &[&Any]) ->STARPARAPKG;
    fn CallBinBuf(&self,FuncName: &Any, Args: &[&Any]) ->STARBINBUF;
    fn Call_Internal(&self,FuncName: &Any, Args: &[&Any]) ->STARRESULT;    
    fn New(&self,args: &[&Any]) -> STAROBJECT;
    fn Free(&self);
    fn Dispose(&self);
    fn RawToParaPkg(&self) -> STARPARAPKG;
    fn DeferFree(&self);
    fn IsInFree(&self) -> bool;
    fn GetSourceScript(&self) -> isize;
    fn GetRefEx(&self) -> isize;
    fn GetRefInfo(&self) -> String;
    fn GetLastError(&self) -> isize;
    fn GetLastErrorInfo(&self) -> String;
    fn RegScriptProc_P(&self,ScriptName:&Any, CallBackProc:fn(CleGroup:&STARSRVGROUP,CleService:&STARSERVICE,CleObject:&STAROBJECT,Paras: &[STARRESULT]) -> STARRESULT);
    fn ReleaseOwnerEx(&self) -> bool; 
    fn IsSLock(&self) -> bool;    
}

impl STAROBJECT_TRAIT for STAROBJECT {
    fn IsValid(&self) -> bool {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return false;
	        }
        	let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject ==  0 as *mut c_void {
		        return false;
	        } 
	        return true;
      }
    }    

    fn ToString(&self) -> String {
        let st: String;

        if let Some(ObjData) = STARGETRCREF_STAROBJECT!(self) {
            st = format!("{}", ObjData);
        } else {
            st = "srp object is invalid".to_owned();
        }
        return st;
    }    

    fn Get(&self,InVar: &Any) -> STARRESULT {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return None;
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return None;
            }          
            let Name : String;
            let ParaName : *const c_char;
        	if let Some(fdata) = InVar.downcast_ref::<String>() {
        		Name = fdata.clone();
                ParaName = SRPRustGetStr(&Name, false);
            }else if let Some(fdata) = InVar.downcast_ref::<&str>() {
                Name = (*fdata).to_owned();
                ParaName = SRPRustGetStr(&Name, false);
            }else{
                let (IntValue,ok) = SRPRustGetInt(InVar, false);
                if ok {
                    Name = format!("\"{}\"",IntValue);
                    ParaName = SRPRustGetStr(&Name, false);
                }else{
                    return None;
                }
            }
        	let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return None;
	        }
	        let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject == 0 as *mut c_void {
		        return None;
	        }
        	if (*starrust_GetCharByIndex(ParaName, 0)) == '_' as c_char && (*starrust_GetCharByIndex(ParaName, 1)) == '_' as c_char && (*starrust_GetCharByIndex(ParaName, 2)) == '_' as c_char {
        		//---process object namevalue
        		let Type = Star_SRPI_GetNameValueType(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3));
		        match Type as i32 {
		            SRPPARATYPE_BOOL => 
                    {
                        let mut BoolValue : VS_BOOL = VS_FALSE;
			            Star_SRPI_GetNameBoolValue(SRPInterface, SRPObject,starrust_GetCharByIndex(ParaName, 3), &mut BoolValue,VS_FALSE);
			            return Some(Box::new(FROMVS_BOOL(BoolValue)));
                    },
		            SRPPARATYPE_INT =>
                    {
                        let mut IntValue : i32 = 0;
			            Star_SRPI_GetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), &mut IntValue, 0);
			            return Some(Box::new(IntValue));
                    },
		            SRPPARATYPE_FLOAT =>
                    {
                        let mut FloatValue : f64 = 0.0;
			            Star_SRPI_GetNameFloatValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), &mut FloatValue, 0.0);
			            return Some(Box::new(FloatValue));
                    },
		            SRPPARATYPE_CHARPTR =>
			            return Some(Box::new(SRPRustSetStr(Star_SRPI_GetNameStrValue(SRPInterface, SRPObject,starrust_GetCharByIndex(ParaName, 3), CString::new("").unwrap().as_ptr()), true))),
            		_ =>
			            return None,
		        }
	        }

	        let HashValue : u32 = Star_SRPI_GetHashValue(SRPInterface, ParaName as *mut c_void, vs_string_strlen(ParaName) as u32, 0);
	        match HashValue {
	            0xF0F4CCC1 => //_Service
                {
		            if Name == "_Service" {
			            return Some(Box::new(RustSRPQueryServiceEx(ObjData.ServiceGroupID, &ObjData.ObjectID)));
		            }
                },
	            0xFA037CAB => //_Class
                {
		            if Name == "_Class" {
			            let SRPObjectTemp = Star_SRPI_GetClass(SRPInterface, SRPObject);
			            if SRPObjectTemp == 0 as *mut c_void {
				            return None;
			            }
			            return Some(Box::new(ToStarObject(SRPObjectTemp, SRPInterface, false)));
		            }
                },
	            0x67F0ABC5 => //_ID
                {
		            if Name == "_ID" {
			            let mut IDTemp : VS_UUID = VS_UUID::new();

			            Star_SRPI_GetID(SRPInterface, SRPObject, &mut IDTemp);
			            return Some(Box::new(SRPRustSetStr(Star_SRPI_UuidToString(SRPInterface, &mut IDTemp) as *mut c_char, false)));
		            }
                },
	            0x64756CF2 => //_Name
                {
		            if Name == "_Name" {
			            return Some(Box::new(SRPRustSetStr(Star_SRPI_GetName(SRPInterface, SRPObject), true)));
		            }
	            },
                _ => {},
            }
	        //---check if is object's attribute
            let mut AttributeInfo : VS_ATTRIBUTEINFO = VS_ATTRIBUTEINFO::new();
	        if Star_SRPI_GetAttributeInfoEx(SRPInterface, Star_SRPI_GetClass(SRPInterface, SRPObject), ParaName, &mut AttributeInfo) == VS_TRUE {
		        return SRPObject_AttributeToRustObject(&ObjData, AttributeInfo.AttributeIndex, SRPInterface, AttributeInfo.Type, AttributeInfo.Length, &mut AttributeInfo.StructID, AttributeInfo.Offset as u32, SRPObject as *mut u8, true);
        	} else {
		        Star_SRPI_LuaPushObject(SRPInterface, SRPObject);
		        Star_SRPI_LuaPushString(SRPInterface, ParaName);
		        Star_SRPI_LuaGetTable(SRPInterface, -2);
		        if Star_SRPI_LuaIsNil(SRPInterface, -1) == VS_FALSE {
			        let mut LuaToJavaResult : bool = false;

			        if Star_SRPI_LuaIsFunction(SRPInterface, -1) == VS_TRUE {
				        Star_SRPI_LuaPop(SRPInterface, 2);
				        return None;
			        }
			        let ObjectTemp1 = LuaToRustObject(0, SRPInterface, -1, &mut LuaToJavaResult);
			        Star_SRPI_LuaPop(SRPInterface, 2);
			        if LuaToJavaResult == false {
                        let S = format!("Get Object[{}] Attribute [{}] Error", SRPRustSetStr(Star_SRPI_GetName(SRPInterface, SRPObject),true), Name);
                        RustPrintError(
                            VSFAULT_WARNING,
                            CString::new(S)
                            .unwrap()
                            .as_ptr(),
                        );                        
        				return None;
			        }
			        return ObjectTemp1;
		        }
            }
		    Star_SRPI_LuaPop(SRPInterface, 2);
	        return None;            
	    }
    }   

   fn GetBool(&self,InVar: &Any) -> bool {
        let RetVal = self.Get(InVar);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<bool>() {
                return *fdata;
            }else{
                return false;
            }
        }else{
            return false;
        }
    }

    fn GetInt(&self,InVar: &Any) -> i32 {
        let RetVal = self.Get(InVar);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToInt(vv,true) as i32;
        }else{
            return 0;
        }
    }

    fn GetInt64(&self,InVar: &Any) -> i64 {
        let RetVal = self.Get(InVar);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToInt64(vv,true) as i64;
        }else{
            return 0;
        }
    }

    fn GetString(&self,InVar: &Any) -> String {
        let RetVal = self.Get(InVar);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<String>() {
                return fdata.clone();
            }else{
                return "".to_owned();
            }
        }else{
            return "".to_owned();
        }
    }

    fn GetDouble(&self,InVar: &Any) -> f64 {
        let RetVal = self.Get(InVar);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToFloat(vv) as f64;
        }else{
            return 0.0;
        }
    }

    fn GetObject(&self,InVar: &Any)->STAROBJECT {
        let RetVal = self.Get(InVar);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STAROBJECT>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn GetParaPkg(&self,InVar: &Any) -> STARPARAPKG {
        let RetVal = self.Get(InVar);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARPARAPKG>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn GetBinBuf(&self,InVar: &Any) -> STARBINBUF {
        let RetVal = self.Get(InVar);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARBINBUF>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }      

    fn Set(&self,InVar: &Any, Para: &Any) {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return;
            }          
            let Name : String;
            let ParaName : *const c_char;
        	if let Some(fdata) = InVar.downcast_ref::<String>() {
        		Name = fdata.clone();
                ParaName = SRPRustGetStr(&Name, false);
            }else if let Some(fdata) = InVar.downcast_ref::<&str>() {
                Name = (*fdata).to_owned();
                ParaName = SRPRustGetStr(&Name, false);
            }else{
                let (IntValue,ok) = SRPRustGetInt(InVar, false);
                if ok {
                    Name = format!("\"{}\"",IntValue);
                    ParaName = SRPRustGetStr(&Name, false);
                }else{
                    return;
                }
            }        
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
        	if SRPInterface == 0 as *mut c_void {
		        return;
	        }
	        let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID.clone());
	        if SRPObject == 0 as *mut c_void {
		        return;
        	}
            if (*starrust_GetCharByIndex(ParaName, 0)) == '_' as c_char && (*starrust_GetCharByIndex(ParaName, 1)) == '_' as c_char && (*starrust_GetCharByIndex(ParaName, 2)) == '_' as c_char {
        		//---process object namevalue
                if let Some(fdata) = Para.downcast_ref::<bool>() {
        			Star_SRPI_SetNameBoolValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), TOVS_BOOL(*fdata), VS_FALSE);
        			return;
                }else if let Some(fdata) = Para.downcast_ref::<i8>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return;
                }else if let Some(fdata) = Para.downcast_ref::<u8>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return;
                }else if let Some(fdata) = Para.downcast_ref::<i16>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return;
                }else if let Some(fdata) = Para.downcast_ref::<u16>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return;   
                }else if let Some(fdata) = Para.downcast_ref::<i32>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return;   
                }else if let Some(fdata) = Para.downcast_ref::<u32>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return; 
                }else if let Some(fdata) = Para.downcast_ref::<isize>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return;    
                }else if let Some(fdata) = Para.downcast_ref::<usize>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return; 
                }else if let Some(fdata) = Para.downcast_ref::<i64>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return;     
                }else if let Some(fdata) = Para.downcast_ref::<u64>() {
        			Star_SRPI_SetNameIntValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as i32, VS_FALSE);
        			return;         
                }else if let Some(fdata) = Para.downcast_ref::<f32>() {
        			Star_SRPI_SetNameFloatValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as f64, VS_FALSE);
        			return;     
                }else if let Some(fdata) = Para.downcast_ref::<f64>() {
        			Star_SRPI_SetNameFloatValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), (*fdata) as f64, VS_FALSE);
        			return;      
                }else if let Some(fdata) = Para.downcast_ref::<String>() {
                    let cstr = SRPRustGetStr(fdata, true);
        			Star_SRPI_SetNameStrValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), cstr, VS_FALSE);
                    Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
        			return;       
                }else if let Some(fdata) = Para.downcast_ref::<&str>() {
                    let cstr = SRPRustGetStr(fdata, true);
        			Star_SRPI_SetNameStrValue(SRPInterface, SRPObject, starrust_GetCharByIndex(ParaName, 3), cstr, VS_FALSE);
                    Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
        			return;                              
                }else{
                    return;
                }           
    		}
        	let HashValue = Star_SRPI_GetHashValue(SRPInterface, ParaName as *mut c_void, vs_string_strlen(ParaName) as u32, 0);
        	match HashValue {
    	        0x64756CF2 => //_Name
            		if Name == "_Name" {
		            	let cstr = SRPRustGetStr(Para, true);
        	    		Star_SRPI_SetName(SRPInterface, SRPObject, cstr);
		            	Star_SRPCS_FreeBuf(StarRust_SRPControlInterface, cstr as *mut c_void);
            			return
	    	        },
                _ => {},            
	        }
            let mut AttributeInfo : VS_ATTRIBUTEINFO = VS_ATTRIBUTEINFO::new();
            if OBJECT_CREATE_ATTACH_BUF == 0 as *const c_char {
                let tt :[c_char;10240] = mem::zeroed();
                OBJECT_CREATE_ATTACH_BUF = Box::into_raw(Box::new(tt)) as *const c_char;
            }
	        if Star_SRPI_GetAttributeInfoEx(SRPInterface,Star_SRPI_GetClass(SRPInterface, SRPObject), ParaName, &mut AttributeInfo) == VS_TRUE {
                if SRPObject_RustObjectToAttribute(SRPInterface, AttributeInfo.Type, AttributeInfo.Length, &AttributeInfo.StructID, Para, 0, OBJECT_CREATE_ATTACH_BUF as *const u8) == false {
                    let S = format!("Set Object[{}] Attribute [{}] Error", SRPRustSetStr(Star_SRPI_GetName(SRPInterface, SRPObject),true), SRPRustSetStr(&mut AttributeInfo.Name[0] as *mut c_char,false));
                    RustPrintError(
                        VSFAULT_WARNING,
                        CString::new(S)
                        .unwrap()
                        .as_ptr(),
                    );                      
		            return;
        		}
        		Star_SRPI_ChangeObject(SRPInterface, SRPObject, AttributeInfo.AttributeIndex, OBJECT_CREATE_ATTACH_BUF as  *const i8);
		        return;
        	} else {
        		Star_SRPI_LuaPushObject(SRPInterface, SRPObject);
		        Star_SRPI_LuaPushString(SRPInterface, ParaName);
        		Star_SRPI_LuaGetTable(SRPInterface, -2);
		        if Star_SRPI_LuaIsNil(SRPInterface, -1) == VS_FALSE {  /*--defined at other scripts--*/
        			let mut DefinedClassID : VS_UUID = VS_UUID::new();

        			Star_SRPI_LuaGetDefinedClass(SRPInterface, SRPObject, &mut DefinedClassID);
			        if Star_SRPI_LuaIsFunction(SRPInterface, -1) == VS_FALSE && starrust_uuidisequal(&ObjData.ObjectID, &DefinedClassID) == VS_TRUE {
        				Star_SRPI_LuaPop(SRPInterface, 2);
				        Star_SRPI_LuaPushObject(SRPInterface, SRPObject);
				        Star_SRPI_LuaPushString(SRPInterface, ParaName);

				        if RustObjectToLua(SRPInterface, Para, false) == false {
                            let S = format!("Set Object[{}] Attribute [{}] Error", SRPRustSetStr(Star_SRPI_GetName(SRPInterface, SRPObject),true), Name);
                            RustPrintError(
                                VSFAULT_WARNING,
                                CString::new(S)
                                .unwrap()
                                .as_ptr(),
                            );     
				        }
        				Star_SRPI_LuaSetTable(SRPInterface, -3);
		        		Star_SRPI_LuaPop(SRPInterface, 1);
				        return;
			        }
		        }
		        Star_SRPI_LuaPop(SRPInterface, 2);
                /*---callback function should be set with RegScriptProc_P --*/
                if Star_SRPI_GetRawLuaSetValueFunc(SRPInterface, SRPObject, ParaName, 0 as *mut usize) != 0 as *mut c_void {
			        Star_SRPI_LuaPushObject(SRPInterface, SRPObject);
			        Star_SRPI_LuaPushString(SRPInterface, ParaName);

			        if RustObjectToLua(SRPInterface, Para, false) == false {
                        let S = format!("Set Object[{}] Attribute [{}] Error", SRPRustSetStr(Star_SRPI_GetName(SRPInterface, SRPObject),true), Name);
                        RustPrintError(
                            VSFAULT_WARNING,
                            CString::new(S)
                            .unwrap()
                            .as_ptr(),
                        ); 
			        }
			        Star_SRPI_LuaSetTable(SRPInterface, -3);
			        Star_SRPI_LuaPop(SRPInterface, 1);
			        return;
		        }

                let S = format!("Set Object[{}] Attribute [{}] Error", SRPRustSetStr(Star_SRPI_GetName(SRPInterface, SRPObject),true), Name);
                RustPrintError(
                    VSFAULT_WARNING,
                    CString::new(S)
                    .unwrap()
                    .as_ptr(),
                );                  
			    return
		    }
	    }
    }    

    fn Call(&self,FuncName: &Any, Args: &[&Any]) -> STARRESULT {
	    return self.Call_Internal(FuncName, Args);
    }

    fn CallBool(&self,FuncName: &Any, Args: &[&Any]) -> bool {
        let RetVal = self.Call(FuncName,Args);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<bool>() {
                return *fdata;
            }else{
                return false;
            }
        }else{
            return false;
        }        
    }

    fn CallInt(&self,FuncName: &Any, Args: &[&Any]) -> i32 {
        let RetVal = self.Call(FuncName,Args);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToInt(vv,true) as i32;
        }else{
            return 0;
        }
    }

    fn CallInt64(&self,FuncName: &Any, Args: &[&Any]) -> i64 {
    	let RetVal = self.Call(FuncName,Args);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToInt64(vv,true) as i64;
        }else{
            return 0;
        }
    }

    fn CallString(&self,FuncName: &Any, Args: &[&Any]) ->String {
	    let RetVal = self.Call(FuncName,Args);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<String>() {
                return fdata.clone();
            }else{
                return "".to_owned();
            }
        }else{
            return "".to_owned();
        }
    }

    fn CallDouble(&self,FuncName: &Any, Args: &[&Any]) ->f64 {
	    let RetVal = self.Call(FuncName,Args);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            return SRPRustToFloat(vv) as f64;
        }else{
            return 0.0;
        }
    }

    fn CallObject(&self,FuncName: &Any, Args: &[&Any]) ->STAROBJECT {
	    let RetVal = self.Call(FuncName,Args);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STAROBJECT>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn CallParaPkg(&self,FuncName: &Any, Args: &[&Any]) ->STARPARAPKG {
	    let RetVal = self.Call(FuncName,Args);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARPARAPKG>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn CallBinBuf(&self,FuncName: &Any, Args: &[&Any]) ->STARBINBUF {
	    let RetVal = self.Call(FuncName,Args);
        if let Some(v) = RetVal.as_ref() { // Box<>
            let  vv = v.as_ref();
            if let Some(fdata) = vv.downcast_ref::<STARBINBUF>() {
                return fdata.clone();
            }else{
                return STARRC!(None);
            }
        }else{
            return STARRC!(None);
        }
    }

    fn Call_Internal(&self,FuncName: &Any, Args: &[&Any]) ->STARRESULT {
        unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return None;
	        }
            let ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return None;
            }       
            let argc = Args.len();   
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return None;
	        }
	        let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID.clone());
	        if SRPObject == 0 as *mut c_void {
		        return None;
	        }

	        let n = Star_SRPI_LuaGetTop(SRPInterface);
	        for i in 0..argc {
		        let localobject = Args[i];
		        RustObjectToLua(SRPInterface, localobject, false);
	        }

	        let FunctionName = SRPRustGetStr(FuncName, false);

	        if Star_SRPI_LuaCall(SRPInterface, SRPObject, FunctionName, argc as i32, -1) == VS_FALSE {
		        let m = Star_SRPI_LuaGetTop(SRPInterface);
		        if m > n {
			        Star_SRPI_LuaPop(SRPInterface, m-n);
		        }
		        return None;
	        }
	        let m = Star_SRPI_LuaGetTop(SRPInterface);
	        if m == n {
		        return None;
	        } else if m > n {
		        if m-n == 1 {
                    let mut RetResult : bool = false;
			        let RetValue = LuaToRustObject(0, SRPInterface, -1, &mut RetResult);
			        Star_SRPI_LuaPop(SRPInterface, m-n);
			        return RetValue;
		        } else {
                    let mut RetResult : bool = false;
                    let mut RetValue : STARRESULT_TUPLE = Vec::new();
        			for i in 0..m-n {
				        RetValue.push(LuaToRustObject(0, SRPInterface, i-(m-n), &mut RetResult));
			        }
			        Star_SRPI_LuaPop(SRPInterface, m-n);
			        return Some(Box::new(RetValue));
        		}
	        } else {
                let S = format!("call function[{:?}] failed,lua stack corrupted", FuncName);
                RustPrintError(
                    VSFAULT_WARNING,
                    CString::new(S)
                    .unwrap()
                    .as_ptr(),
                );                 
		        return None;
	        }
        }  
    }  

    fn New(&self,args: &[&Any]) -> STAROBJECT {
    	let ObjData: &StarServiceBody;
        let argc : i32 = args.len() as i32;
    	let mut QueueAttrName : *mut c_char;
	    let mut AttributeChangeString : *mut c_char;
    	let mut ObjectNameString : *mut c_char;
	    let mut ObjectTemp : &Any;
    	let SRPObject : *mut c_void;
	    let mut SRPParentObject : *mut c_void;
	    let mut Index : i32 = 0;
    	let i : i32;
	    let AttributeNumber : i32;
        let mut SkipObjectNameString : bool = false;

      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return STARRC!(None);
            }       

        	QueueAttrName = 0 as *mut c_char;
        	SRPParentObject = 0 as *mut c_void;
        	AttributeChangeString = 0 as *mut c_char;
        	ObjectNameString = 0 as *mut c_char;
	        ObjectTemp = &(0 as isize);

        	if argc == 0 {
        		QueueAttrName = 0 as *mut c_char;
		        SRPParentObject = 0 as *mut c_void;
        	} else {
		        ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
		        if !AnyIsZero(ObjectTemp) {     
                    if ObjectTemp.is::<String>() || ObjectTemp.is::<&str>() {
           				QueueAttrName = SRPRustGetStr(ObjectTemp,false);
           				Index = Index + 1;
	            		ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
        				if AnyIsZero(ObjectTemp) { //no more parameter
		        			ObjectNameString = QueueAttrName;
				        	QueueAttrName = 0 as *mut c_char;
					        SRPParentObject = 0 as *mut c_void;
                            SkipObjectNameString = true;
        				} else {
		        			if IsStarObjectClassObject(ObjectTemp) == true {
				        		SRPParentObject = RustObjectToSRPObject(ObjectTemp);
        						Index = Index + 1;
		        				ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
    					        if ObjectTemp.is::<String>() || ObjectTemp.is::<&str>() {                                
            						ObjectNameString = QueueAttrName;
            						QueueAttrName = 0 as *mut c_char;
            						SRPParentObject =  0 as *mut c_void;
						            AttributeChangeString = SRPRustGetStr(ObjectTemp, true);
                                    Index = Index + 1;
                                    SkipObjectNameString = true;
            						ObjectTemp = SRPObject_GetArrayObject(argc, args, Index) //end
					            } else {
						            return STARRC!(None);
        					    }
            				}
                        }
			        } else if IsStarObjectClassObject(ObjectTemp) == true {
        				SRPParentObject = RustObjectToSRPObject(ObjectTemp);
				        Index = Index + 1;
				        ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
        			} else {
		        		return STARRC!(None);
        			}
		        }
	        }
        	if !AnyIsZero(ObjectTemp) && SkipObjectNameString == false {
		        if ObjectTemp.is::<String>() || ObjectTemp.is::<&str>() {
        			ObjectNameString = SRPRustGetStr(ObjectTemp, false);     
        			Index = Index + 1;
        			ObjectTemp = SRPObject_GetArrayObject(argc, args, Index);
			        if AnyIsZero(ObjectTemp) { //no more parameter     
        			} else if ObjectTemp.is::<String>() || ObjectTemp.is::<&str>() {
        				AttributeChangeString = SRPRustGetStr(ObjectTemp, true);
				        Index = Index + 1;
				        ObjectTemp = SRPObject_GetArrayObject(argc, args, Index); //end                        
        			}                                              
        		}
	        }

        	let mut InitBuf : *const c_void = 0 as *const c_void;
	        if Index < argc {
        		let BasicSRPInterface = RustSRPGetBasicSRPInterface(ObjData.ServiceGroupID);
		        let SRPInterface = GetSRPServiceInterface(ObjData.ServiceGroupID, 0 as *const c_void);
		        InitBuf = Star_SRPBasic_GetParaPkgInterface(BasicSRPInterface);
		        StarParaPkg_FromTuple_Sub(args, InitBuf as *mut c_void, BasicSRPInterface, Index, SRPInterface);
	        }
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
        	if SRPInterface == 0 as *mut c_void {
                RustPrintError(
                    VSFAULT_WARNING,
                    CString::new("Get Interface Error".to_owned())
                    .unwrap()
                    .as_ptr(),
                );                          
        		Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
		        if InitBuf != 0 as *const c_void {
			        Star_SRPParaPkg_Release(InitBuf);
		        }
		        return STARRC!(None);
	        }
	        SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject == 0 as *mut c_void {
                RustPrintError(
                    VSFAULT_WARNING,
                    CString::new("Get Object Error".to_owned())
                    .unwrap()
                    .as_ptr(),
                );                  
        		Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
		        if InitBuf != 0 as *const c_void {
			        Star_SRPParaPkg_Release(InitBuf);
		        }
		        return STARRC!(None);
        	}

	        let RetValue = SRPObject_CreateNewObject(SRPInterface, SRPObject, 4/*C.VSALLOCTYPE_LOCAL*/, 0 as *mut VS_UUID, 0, QueueAttrName, SRPParentObject, ObjectNameString, AttributeChangeString, InitBuf);
	        if RetValue == 0 as *const c_void {
        		Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
		        if InitBuf != 0 as *const c_void {
			        Star_SRPParaPkg_Release(InitBuf);
		        }
		        return STARRC!(None);
	        }
        	Star_SRPCS_FreeBuf(StarRust_SRPCoreShellInterface, AttributeChangeString as *mut c_void);
		    if InitBuf != 0 as *const c_void {
		        Star_SRPParaPkg_Release(InitBuf);
		    }
	        return ToStarObject(RetValue as *mut c_void, SRPInterface, true);
        }
    }    

    fn Free(&self) {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let mut In_ObjData = STARGETRCMUT_STAROBJECT!(self);
            if let Some(ObjData) = In_ObjData.as_mut() {   
                let mut fdata = &mut ObjData.ObjData;         
            	let SRPInterface = GetSRPServiceInterfaceEx(fdata.ServiceGroupID, &fdata.ObjectID);
	            if SRPInterface == 0 as *mut c_void {
		            return;
    	        }
	            let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut fdata.ObjectID);
	            if SRPObject == 0 as *mut c_void {
		            return;
    	        }
                //_TermObject_Defer(&mut fdata);
            	{
                    if fdata.IsClearedByStarCore == false {
                        if StarRust_ModuleInitFlag == VS_TRUE {
                            RustSRPClearObject(fdata.ServiceGroupID, &mut fdata.ObjectID);

                            Star_SRPI_UnRegLuaFuncEx(SRPInterface,SRPObject as *mut c_void,starrust_SRPObject_ScriptCallBack as *mut c_void,fdata.RefItem);
                            if fdata.NeedFreeFlag == true {
                                Star_SRPI_UnLockGC(SRPInterface, SRPObject as *mut c_void);
                            }
                        }
                    }
                    fdata.IsClearedByStarCore = true;
                    if StarRust_ModuleInitFlag == VS_TRUE && StarRust_SRPControlInterface != 0 as *mut c_void && fdata.RefItem != 0 {
                        Star_SRPControl_UnRegScriptObject(
                            StarRust_SRPControlInterface,
                            fdata.RefItem,
                            starrust_FreeScriptObject as *mut c_void,
                            0,
                        );
                    }
                    if fdata.RefItem != 0 {
                        DeleteRustObjectAllRef(fdata.RefItem);
                        fdata.RefItem = 0;
                    }
                }
                /*--for free object trigger callback, which cause borrow panic, so we should clear context before free object--*/
	            Star_SRPI_FreeObject(SRPInterface, SRPObject);                   
	            return;
            }else{
                return;
            }
        }
    }

    fn Dispose(&self) {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let mut In_ObjData = STARGETRCMUT_STAROBJECT!(self);
            if let Some(ObjData) = In_ObjData.as_mut() {   
                let mut fdata = &mut ObjData.ObjData; 
               
            	let SRPInterface = GetSRPServiceInterfaceEx(fdata.ServiceGroupID, &fdata.ObjectID);
	            if SRPInterface == 0 as *mut c_void {
		            return;
	            }
    	        let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut fdata.ObjectID);
	            if SRPObject == 0 as *mut c_void {
		            return;
	            }
    	        //_TermObject_Defer(&mut fdata);
            	{
                    if fdata.IsClearedByStarCore == false {
                        if StarRust_ModuleInitFlag == VS_TRUE {
                            RustSRPClearObject(fdata.ServiceGroupID, &mut fdata.ObjectID);

                            Star_SRPI_UnRegLuaFuncEx(SRPInterface,SRPObject as *mut c_void,starrust_SRPObject_ScriptCallBack as *mut c_void,fdata.RefItem);
                            if fdata.NeedFreeFlag == true {
                                Star_SRPI_UnLockGC(SRPInterface, SRPObject as *mut c_void);
                            }
                        }
                    }
                    fdata.IsClearedByStarCore = true;
                    if StarRust_ModuleInitFlag == VS_TRUE && StarRust_SRPControlInterface != 0 as *mut c_void && fdata.RefItem != 0 {
                        Star_SRPControl_UnRegScriptObject(
                            StarRust_SRPControlInterface,
                            fdata.RefItem,
                            starrust_FreeScriptObject as *mut c_void,
                            0,
                        );
                    }
                    if fdata.RefItem != 0 {
                        DeleteRustObjectAllRef(fdata.RefItem);
                        fdata.RefItem = 0;
                    }
                }                
	            return;
            }else{
                return;
            }
        }   
    } 

    fn RawToParaPkg(&self) -> STARPARAPKG {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return STARRC!(None);
            }    
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return STARRC!(None);
	        }
        	let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject ==  0 as *mut c_void {
		        return STARRC!(None);
	        }
	        let ParaPkg = Star_SRPI_RawToParaPkg(SRPInterface, SRPObject);
	        if ParaPkg == 0 as *mut c_void {
		        return STARRC!(None);
	        }
	        return ToStarParaPkg(ParaPkg, ObjData.ServiceGroupID, false); //--not create by self;
        }
    }

    fn DeferFree(&self) {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let mut In_ObjData = STARGETRCMUT_STAROBJECT!(self);
            if let Some(ObjData) = In_ObjData.as_mut() {   
                let mut fdata = &mut ObjData.ObjData;         
            	let SRPInterface = GetSRPServiceInterfaceEx(fdata.ServiceGroupID, &fdata.ObjectID);
	            if SRPInterface == 0 as *mut c_void {
		            return;
    	        }
	            let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut fdata.ObjectID);
	            if SRPObject == 0 as *mut c_void {
		            return;
    	        }
                //_TermObject_Defer(&mut fdata);
            	{
                    if fdata.IsClearedByStarCore == false {
                        if StarRust_ModuleInitFlag == VS_TRUE {
                            RustSRPClearObject(fdata.ServiceGroupID, &mut fdata.ObjectID);

                            Star_SRPI_UnRegLuaFuncEx(SRPInterface,SRPObject as *mut c_void,starrust_SRPObject_ScriptCallBack as *mut c_void,fdata.RefItem);
                            if fdata.NeedFreeFlag == true {
                                Star_SRPI_UnLockGC(SRPInterface, SRPObject as *mut c_void);
                            }
                        }
                    }
                    fdata.IsClearedByStarCore = true;
                    if StarRust_ModuleInitFlag == VS_TRUE && StarRust_SRPControlInterface != 0 as *mut c_void && fdata.RefItem != 0 {
                        Star_SRPControl_UnRegScriptObject(
                            StarRust_SRPControlInterface,
                            fdata.RefItem,
                            starrust_FreeScriptObject as *mut c_void,
                            0,
                        );
                    }
                    if fdata.RefItem != 0 {
                        DeleteRustObjectAllRef(fdata.RefItem);
                        fdata.RefItem = 0;
                    }
                }
	            Star_SRPI_DeferFreeObject(SRPInterface, SRPObject);                   
	            return;
            }else{
                return;
            }
        }
    }    

    fn IsInFree(&self) -> bool {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return false;
	        }
        	let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject ==  0 as *mut c_void {
		        return false;
	        }            
	        return FROMVS_BOOL(Star_SRPI_IsObjectInFree(SRPInterface, SRPObject));
        }
    }

    fn GetSourceScript(&self) -> isize {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return -1;
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return -1;
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return -1;
	        }
        	let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject ==  0 as *mut c_void {
		        return -1;
	        }            
	        return Star_SRPI_GetSourceScript(SRPInterface, SRPObject) as isize;
        }
    }

    fn GetRefEx(&self) -> isize {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return 0;
	        }
        	let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject ==  0 as *mut c_void {
		        return 0;
	        } 
	        return Star_SRPI_GetRefEx(SRPInterface, SRPObject) as isize;
        }
    }

    fn GetRefInfo(&self) -> String {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
        	let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject ==  0 as *mut c_void {
		        return "".to_owned();
	        } 
	        let CharPtr = Star_SRPI_GetRefInfo(SRPInterface, SRPObject);
	        if CharPtr == 0 as *mut c_char {
		        return "".to_owned();
	        }
	        return SRPRustSetStr(CharPtr, false);
        }
    }

    fn GetLastError(&self) -> isize {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return 0;
	        }
            let ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return 0;
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return 0;
	        }
	        return Star_SRPI_GetLastError(SRPInterface) as isize;
        }    
    }

    fn GetLastErrorInfo(&self) -> String {
     unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return "".to_owned();
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return "".to_owned();
	        }
            let mut LineIndex : u32 = 0;
            let mut SourceName : Vec<*mut c_char> = Vec::new();
            SourceName.push(0 as *mut c_char);
	        let TextBuf = Star_SRPI_GetLastErrorInfo(SRPInterface, &mut LineIndex,SourceName.as_mut_ptr() as usize as *mut [*mut c_char;1]);
	        return format!("[{}:{}]{}", SRPRustSetStr(SourceName[0],false), LineIndex, SRPRustSetStr(TextBuf, true));
        }

    }

    fn RegScriptProc_P(&self,ScriptName:&Any, CallBackProc:fn(CleGroup:&STARSRVGROUP,CleService:&STARSERVICE,CleObject:&STAROBJECT,Paras: &[STARRESULT]) -> STARRESULT) {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return;
	        }
            let PostObjData : StarObjectBody;
            {
                let mut In_ObjData = STARGETRCMUT_STAROBJECT!(self);
                if let Some(ObjData) = In_ObjData.as_mut() {   
                    let mut fdata = &mut ObjData.ObjData; 
                     PostObjData = fdata.clone();

                	let SRPInterface = GetSRPServiceInterfaceEx(fdata.ServiceGroupID, &fdata.ObjectID);
	                if SRPInterface == 0 as *mut c_void {
		                return;
	                }
    	            let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut fdata.ObjectID);
    	            if SRPObject == 0 as *mut c_void {
	    	            return;
	                }        
                    let ParaName = SRPRustGetStr(ScriptName, false);
		            Star_SRPI_RegLuaFunc(SRPInterface, SRPObject, ParaName, starrust_SRPObject_ScriptCallBack as *mut c_void, fdata.RefItem);
       				//RustSRPSetObject(fdata.ServiceGroupID, &fdata.ObjectID, self, true);
                    fdata.FuncTable.insert(SRPRustSetStr(ParaName, false), CallBackProc);
                    STARRUST_SAFERELEASESTR(ScriptName, ParaName);
                }else{
                    return;
                }          
            }
            RustSRPSetObject(PostObjData.ServiceGroupID, &PostObjData.ObjectID, self, true);    
        }
    }

    fn ReleaseOwnerEx(&self) -> bool {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return false;
	        }
        	let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject ==  0 as *mut c_void {
		        return false;
	        }         
	        if RustSRPIsObjectGlobalRef(ObjData.ServiceGroupID, &mut ObjData.ObjectID) == false {
		        return false;
	        }
	        return FROMVS_BOOL(Star_SRPI_ReleaseOwnerExForScript(SRPInterface, CString::new("rust".to_owned()).unwrap().as_ptr(),SRPObject));
        }
    }

    fn IsSLock(&self) -> bool {
      unsafe{
	        if StarRust_SRPControlInterface == 0 as *mut c_void {
		        return false;
	        }
            let mut ObjData : StarObjectBody;
            if let Some(fbody) = STARGETRCREF_STAROBJECT!(self) {    
                ObjData = fbody.ObjData.clone();
            }else{
                return false;
            }  
	        let SRPInterface = GetSRPServiceInterfaceEx(ObjData.ServiceGroupID, &ObjData.ObjectID);
	        if SRPInterface == 0 as *mut c_void {
		        return false;
	        }
        	let SRPObject = Star_SRPI_GetObject(SRPInterface, &mut ObjData.ObjectID);
	        if SRPObject ==  0 as *mut c_void {
		        return false;
	        }         
	        if RustSRPIsObjectGlobalRef(ObjData.ServiceGroupID, &mut ObjData.ObjectID) == false {
		        return false;
	        }        
	        return RustSRPIsObjectGlobalRef(ObjData.ServiceGroupID, &mut ObjData.ObjectID);
        }
    }
}

#[no_mangle]
pub extern "C" fn RustSRPObject_ScriptCallBack(L: *const c_void) -> i32 {
unsafe{    
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|panic_info| {
            let (filename, line) = panic_info.location().map(|loc| (loc.file(), loc.line())).unwrap_or(("<unknown>", 0));
            let cause = panic_info.payload().downcast_ref::<String>().map(String::as_ref);
            let cause = cause.unwrap_or_else(||
                        panic_info.payload().downcast_ref::<&str>().map(|s| *s).unwrap_or("<cause unknown>")
                );
            let s = format!("A panic occurred at {}:{}: {}", filename, line, cause);
            starrust_SRPControl_ProcessError(
                    StarRust_SRPControlInterface,
                    VSFAULT_WARNING,
                    CString::new("rust").unwrap().as_ptr(),
                    0,
                    CString::new(s).unwrap().as_ptr(),
                );
        }));
        let callresult = panic::catch_unwind(AssertUnwindSafe(|| {  
    //-----------------------------------------------------------------------------
	let SRPInterface : *mut c_void;
    let CallBackProc: fn(CleGroup:&STARSRVGROUP,CleService:&STARSERVICE,CleObject:&STAROBJECT,Paras: &[STARRESULT]) -> STARRESULT;

	let ServiceGroupID = Star_SRPControl_LuaGetInt(StarRust_SRPControlInterface, L as *mut c_void, Star_SRPControl_LuaUpValueIndex(StarRust_SRPControlInterface, L as *mut c_void, 1));

    let ScriptName: *mut c_char;
    let Object : *mut c_void;
    let RustScriptName : String;

	let ObjectTemp = SRefToRustObject_StarObject(Star_SRPControl_LuaGetUWord(StarRust_SRPControlInterface, L as *mut c_void, Star_SRPControl_LuaUpValueIndex(StarRust_SRPControlInterface, L as *mut c_void, 2)));
    if let Some(fbody) = STARGETRCREF_STAROBJECT!(ObjectTemp) {    
        let selfVal = fbody.ObjData.clone();
        SRPInterface = GetSRPServiceInterfaceEx(selfVal.ServiceGroupID, &selfVal.ObjectID);
        if SRPInterface == 0 as *mut c_void {
	        return 0;
        }        
    	ScriptName = Star_SRPI_LuaToString(SRPInterface, Star_SRPI_LuaUpValueIndex(SRPInterface, 3));
        RustScriptName = SRPRustSetStr(ScriptName,false);
	    Object = Star_SRPI_LuaToObject(SRPInterface, 1);      
        if fbody.ObjData.FuncTable.contains_key(&RustScriptName) {
            CallBackProc = *fbody.ObjData.FuncTable.get(&RustScriptName).unwrap();
        }else{
            return 0;
        }        
    }else{
        return 0;
    }  
	if Star_SRPI_IsRemoteCall(SRPInterface, Object) == VS_TRUE {
		Star_SRPI_SetRetCode(SRPInterface, Object, 0xFFFFFFFC as u32 /*VSRCALL_PARAERROR*/);
	}
	//---create parameter
	if Star_SRPI_LuaGetTop(SRPInterface) == 0 {
        let S = format!("Call Object[{}] RustFunction [{}] Error Parameter Number ", SRPRustSetStr(Star_SRPI_GetName(SRPInterface, Object),false),SRPRustSetStr(ScriptName,false));
        RustPrintError(
            VSFAULT_WARNING,
            CString::new(S)
            .unwrap()
            .as_ptr(),
        );
		return 0;
	}
    let mut n = Star_SRPI_LuaGetTop(SRPInterface) - 1;
    let mut ScriptParaArray: Vec<STARRESULT> = Vec::with_capacity(n as usize);
	let mut ObjectTemp2: STARRESULT;
	let mut LuaToJavaResult: bool = false;
	for i in 0..n {
		ObjectTemp2 = LuaToRustObject(0, SRPInterface, i+2, &mut LuaToJavaResult);
		if LuaToJavaResult == false {
            let S = format!("Call Object[{}] RustFunction [{}] Error Parameter [{}] Number ", SRPRustSetStr(Star_SRPI_GetName(SRPInterface, Object),false),SRPRustSetStr(ScriptName,false),i);
            RustPrintError(
                VSFAULT_WARNING,
                CString::new(S)
                .unwrap()
                .as_ptr(),
            );
	    	return 0;            
		}
		ScriptParaArray.push(ObjectTemp2);
	}
    /*---*/
    let StarSrvGroup_ObjectTemp : STARSRVGROUP = RustSRPQuerySrvGroup(ServiceGroupID);
    {
        if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(StarSrvGroup_ObjectTemp){

        }else{
            return 0;
        }
    }
    let StarService_ObjectTemp : STARSERVICE = RustSRPQueryService(ServiceGroupID, Object);
    {
        if let Some(ObjData) = STARGETRCREF_STARSERVICE!(StarService_ObjectTemp){

        }else{
            return 0;
        }
    }    
	let RetValue = CallBackProc(&StarSrvGroup_ObjectTemp,&StarService_ObjectTemp,&ObjectTemp, &ScriptParaArray[..]);
	Star_SRPI_SetRetCode(SRPInterface, Object, 0/*VSRCALL_OK*/);
    match RetValue.Type() {
        STARRESULTTYPE::T_NONE => return 0,
        _ => {
		    n = Star_SRPI_LuaGetTop(SRPInterface);
            SetReturnValue(SRPInterface,&RetValue);
		    return Star_SRPI_LuaGetTop(SRPInterface) - n;
	    }
    }
    //-----------------------------------------------------------------------------
        }));
    panic::set_hook(old_hook);
    if callresult.is_err() {
        return 0;
    }else{    
        let vv = callresult.ok().expect("");
        return vv;
    }   
}    
}

fn SetReturnValue(SRPInterface : *mut c_void,RetValue: &STARRESULT)
{
    match RetValue.Type() {
        STARRESULTTYPE::T_BOOL => 
        {
            let s = RetValue.ToBool();
            RustObjectToLua(SRPInterface, &s, false);
        },
        STARRESULTTYPE::T_I8 => 
        {
            let s = RetValue.ToInt();
            RustObjectToLua(SRPInterface, &s, false);
        },
        STARRESULTTYPE::T_U8 => 
        {
            let s = RetValue.ToInt();
            RustObjectToLua(SRPInterface, &s, false);
        },        
        STARRESULTTYPE::T_I16 => 
        {
            let s = RetValue.ToInt();
            RustObjectToLua(SRPInterface, &s, false);
        },  
        STARRESULTTYPE::T_U16 => 
        {
            let s = RetValue.ToInt();
            RustObjectToLua(SRPInterface, &s, false);
        },  
        STARRESULTTYPE::T_I32 => 
        {
            let s = RetValue.ToInt();
            RustObjectToLua(SRPInterface, &s, false);
        },   
        STARRESULTTYPE::T_U32 => 
        {
            let s = RetValue.ToInt();
            RustObjectToLua(SRPInterface, &s, false);
        },   
        STARRESULTTYPE::T_I64 => 
        {
            let s = RetValue.ToInt64();
            RustObjectToLua(SRPInterface, &s, false);
        },     
        STARRESULTTYPE::T_U64 => 
        {
            let s = RetValue.ToInt64();
            RustObjectToLua(SRPInterface, &s, false);
        },     
        STARRESULTTYPE::T_ISIZE => 
        {
            let s = RetValue.ToInt64() as usize as isize;
            RustObjectToLua(SRPInterface, &s, false);
        },     
        STARRESULTTYPE::T_USIZE => 
        {
            let s = RetValue.ToInt64() as usize;
            RustObjectToLua(SRPInterface, &s, false);
        },  
        STARRESULTTYPE::T_F32 => 
        {
            let s = RetValue.ToDouble();
            RustObjectToLua(SRPInterface, &s, false);
        },  
        STARRESULTTYPE::T_F64 => 
        {
            let s = RetValue.ToDouble();
            RustObjectToLua(SRPInterface, &s, false);
        },
        STARRESULTTYPE::T_STRING => 
        {
            let s = RetValue.ToString();
            RustObjectToLua(SRPInterface, &s, false);
        },                      
        STARRESULTTYPE::T_STAROBJECT => 
        {
            let s = RetValue.ToObject();
            RustObjectToLua(SRPInterface, &s, false);
        },     
        STARRESULTTYPE::T_STARPARAPKG => 
        {
            let s = RetValue.ToParaPkg();
            RustObjectToLua(SRPInterface, &s, false);
        }, 
        STARRESULTTYPE::T_STARBINBUF => 
        {
            let s = RetValue.ToBinBuf();
            RustObjectToLua(SRPInterface, &s, false);
        },          
        STARRESULTTYPE::T_STARRESULT_TUPLE =>  /*--return is array--*/
        {
            RustPrintError(
                VSFAULT_WARNING,
                CString::new("return STARRESULT_TUPLE is not supported")
                .unwrap()
                .as_ptr(),
            );
        },                                                                                                                                                            
    _ => {},
    }	
    return;
}

#[cfg(not(feature="star-sharelib"))] 
#[no_mangle]
pub extern "C" fn RustVSScript_InitRaw(Para: u32, BasicSRPInterface: *mut c_void, SRPInterface: *mut c_void) -> VS_BOOL {
unsafe{
    let SrvGroup : STARSRVGROUP;
	let Temp_SrvGroup = RustSRPQuerySrvGroup(Star_SRPBasic_GetServiceGroupID(BasicSRPInterface));  /* STARSRVGROUP */
    {
        if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(Temp_SrvGroup) {  /* -- isvalid--*/
            SrvGroup = Temp_SrvGroup.clone();
        }else{
            SrvGroup = RustSRPGetSrvGroup(Star_SRPBasic_GetServiceGroupID(BasicSRPInterface), BasicSRPInterface);
        }
    }
    let ServiceIsValid : bool;
    let mut Service : STARSERVICE = STARRC!(None);
    {
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(SrvGroup); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {  
            let mut ServiceID : VS_UUID = VS_UUID::new();
            let Temp_Service = RustSRPQueryServiceByServiceID(&mut ObjData.ObjData, &mut ServiceID);
            let STARGETRCMUT_Temp = STARGETRCMUT_STARSERVICE_ToRef(Temp_Service.as_ref()).borrow();
            if let Some(ObjData) = STARGETRCMUT_Temp.as_ref() {  /* -- isvalid--*/
                Service = Temp_Service.clone();
                ServiceIsValid = false;
            }else{        
                Star_SRPI_AddRef(SRPInterface);
                Service = ToStarService(&mut ObjData.ObjData, SRPInterface);  
                ServiceIsValid = true;      
            }
        }else{
            ServiceIsValid = false;
        }
    }
	if ServiceIsValid == true {
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|panic_info| {
            let (filename, line) = panic_info.location().map(|loc| (loc.file(), loc.line())).unwrap_or(("<unknown>", 0));
            let cause = panic_info.payload().downcast_ref::<String>().map(String::as_ref);
            let cause = cause.unwrap_or_else(||
                        panic_info.payload().downcast_ref::<&str>().map(|s| *s).unwrap_or("<cause unknown>")
                );
            let s = format!("A panic occurred at {}:{}: {}", filename, line, cause);
            starrust_SRPControl_ProcessError(
                    StarRust_SRPControlInterface,
                    VSFAULT_WARNING,
                    CString::new("rust").unwrap().as_ptr(),
                    0,
                    CString::new(s).unwrap().as_ptr(),
                );
        }));
        let callresult = panic::catch_unwind(AssertUnwindSafe(|| {        
            if G_SCRIPTINITCALLBACK as *const c_void != Default_ScriptInitCallBack as *const c_void {       
                G_SCRIPTINITCALLBACK(&SrvGroup,&Service);
            }
        }));
        panic::set_hook(old_hook);       
	}
	return VS_TRUE;
}    
}

#[cfg(feature="star-sharelib")]
#[no_mangle]
pub extern "C" fn RustVSScript_InitRaw(Para: u32, BasicSRPInterface: *mut c_void, SRPInterface: *mut c_void) -> VS_BOOL {
unsafe{
    let SrvGroup : STARSRVGROUP;
	let Temp_SrvGroup = RustSRPQuerySrvGroup(Star_SRPBasic_GetServiceGroupID(BasicSRPInterface));  /* STARSRVGROUP */
    {
        if let Some(ObjData) = STARGETRCREF_STARSRVGROUP!(Temp_SrvGroup) {  /* -- isvalid--*/
            SrvGroup = Temp_SrvGroup.clone();
        }else{
            SrvGroup = RustSRPGetSrvGroup(Star_SRPBasic_GetServiceGroupID(BasicSRPInterface), BasicSRPInterface);
        }
    }
    let ServiceIsValid : bool;
    let mut Service : STARSERVICE = STARRC!(None);
    {
        let mut STARGETRCMUT_temp = STARGETRCMUT_STARSRVGROUP!(SrvGroup); 
        if let Some(ObjData) = STARGETRCMUT_temp.as_mut() {  
            let mut ServiceID : VS_UUID = VS_UUID::new();
            let Temp_Service = RustSRPQueryServiceByServiceID(&mut ObjData.ObjData, &mut ServiceID);
            let STARGETRCMUT_Temp = STARGETRCMUT_STARSERVICE_ToRef(Temp_Service.as_ref()).borrow();
            if let Some(ObjData) = STARGETRCMUT_Temp.as_ref() {  /* -- isvalid--*/
                Service = Temp_Service.clone();
                ServiceIsValid = false;
            }else{        
                Star_SRPI_AddRef(SRPInterface);
                Service = ToStarService(&mut ObjData.ObjData, SRPInterface);  
                ServiceIsValid = true;      
            }
        }else{
            ServiceIsValid = false;
        }
    }
	if ServiceIsValid == true {
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|panic_info| {
            let (filename, line) = panic_info.location().map(|loc| (loc.file(), loc.line())).unwrap_or(("<unknown>", 0));
            let cause = panic_info.payload().downcast_ref::<String>().map(String::as_ref);
            let cause = cause.unwrap_or_else(||
                        panic_info.payload().downcast_ref::<&str>().map(|s| *s).unwrap_or("<cause unknown>")
                );
            let s = format!("A panic occurred at {}:{}: {}", filename, line, cause);
            starrust_SRPControl_ProcessError(
                    StarRust_SRPControlInterface,
                    VSFAULT_WARNING,
                    CString::new("rust").unwrap().as_ptr(),
                    0,
                    CString::new(s).unwrap().as_ptr(),
                );
        }));
        let callresult = panic::catch_unwind(AssertUnwindSafe(|| {          
            if G_SCRIPTINITCALLBACK as *const c_void != Default_ScriptInitCallBack as *const c_void {       
                G_SCRIPTINITCALLBACK(&SrvGroup,&Service);
		    }else{
                ScriptInitCallBack(&SrvGroup,&Service);
            }
        }));
        panic::set_hook(old_hook); 
	}
	return VS_TRUE;
}    
}

pub fn Stub_StarCoreService_Init2(StarCore:*const c_void, InterfaceTable:*const c_void ) -> i8
{
    unsafe{
        let res = starrust_StarCoreService_Init2(StarCore,InterfaceTable);
        return res as i8;
    }
}
    
pub fn Stub_StarCoreService_Term2(StarCore:*const c_void, InterfaceTable:*const c_void )
{
    unsafe{
        starrust_StarCoreService_Term2(StarCore,InterfaceTable);
    }
}
