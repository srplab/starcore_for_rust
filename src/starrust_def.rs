/*-----------------------------------------------------------------------
let para = star_parapkg!(StarSrvGroup,1,2,3);
let para = star_binbuf!(StarSrvGroup);

star_fn!(CallBackObj,"PrintHello",CleGroup,CleService,CleObject,a:String, b:i32 {
    });

star_ret!(star_parapkg!(CleGroup,"return from go",345.4));

let retobj = star_call!(python,"tt","hello ","world");
let retobj = star_callbool!(python,"tt","hello ","world");
let retobj = star_callint!(python,"tt","hello ","world");
let retobj = star_callint64!(python,"tt","hello ","world");
let retobj = star_callstring!(python,"tt","hello ","world");
let retobj = star_calldouble!(python,"tt","hello ","world");
let retobj = star_callobject!(python,"tt","hello ","world");
let retobj = star_callparapkg!(python,"tt","hello ","world");
let retobj = star_callbinbuf!(python,"tt","hello ","world");
let retobj = star_get!(python,"tt");
let retobj = star_getbool!(python,"tt");
let retobj = star_getint!(python,"tt");
let retobj = star_getint64!(python,"tt");
let retobj = star_getstring!(python,"tt");
let retobj = star_getdouble!(python,"tt");
let retobj = star_getobject!(python,"tt");
let retobj = star_getparapkg!(python,"tt");
let retobj = star_getbinbuf!(python,"tt");
let retobj = star_set!(python,"g1",456);
let retobj = parapkg_get!(para,0);
let retobj = parapkg_getbool!(para,0);
let retobj = parapkg_getint!(para,0);
let retobj = parapkg_getint64!(para,0);
let retobj = parapkg_getstring!(para,0);
let retobj = parapkg_getdouble!(para,0);
let retobj = parapkg_getobject!(para,0);
let retobj = parapkg_getparapkg!(para,0);
let retobj = parapkg_getbinbuf!(para,0);
let retobj = parapkg_set!(para,0,"2222");

let (retobj,err) = star_runscript!("python","print(123)","","");
let (retobj,err) = star_runscriptex!("python",binbuf,"","");
let (retobj,err) = star_dofile!("python",aaa.py","");
let (retobj,err) = star_dofileex!("python",aaa.py","","");

------------------------------------------------------------------------*/

/// Creates StarParaPkg
/// 
/// # Example
/// let para = star_parapkg!(StarSrvGroup,1,2,3);
/// ```
#[macro_export]
macro_rules! star_parapkg {
    ($GroupID: expr, $($var: expr),*) => {
        $GroupID.NewParaPkg(&[$(&$var),*]);
    };  
}

/// Creates StarBinBuf
/// 
/// # Example
/// 
/// let para = star_binbuf!(StarSrvGroup);
/// ```
#[macro_export]
macro_rules! star_binbuf {
    ($GroupID: expr) => {
        $GroupID.NewBinBuf();
    };
    () => {
        starrust::GetSrvGroup(&0).NewBinBuf();
    };    
}

/// Define Object's function
/// 
/// # Example
/// 
/// let CallBackObj = Service.New(&[]);
/// star_fn!(CallBackObj,"PrintHello",CleGroup,CleService,CleObject,a:String, b:i32 {
///     println!("########{:?}",a);
///     println!("########{:?}",b);
///     star_ret!(star_parapkg!(CleGroup;"return from go",345.4));
/// });
/// ```
#[macro_export]
macro_rules! star_fn {
    ($Obj:ident,$name:expr,$CleGroup:ident,$CleService:ident,$CleObject:ident,$($var:ident : $ptype:ty),* $body:block ) => {
        {
        fn rustcallback($CleGroup:&starrust::STARSRVGROUP,$CleService:&starrust::STARSERVICE,$CleObject:&starrust::STAROBJECT,Paras: &[starrust::STARRESULT]) -> starrust::STARRESULT {
            //$(let $var = star_fn_argparse![Paras[0],stringify!($ptype)];)* 
            let mut Paras_Index : isize = -1; 
            $( let $var : $ptype;
               {
                   Paras_Index = Paras_Index + 1;
                   if let Some(v) = Paras[Paras_Index as usize].as_ref() { 
                       let  vv = v.as_ref();
                       if let Some(fdata) = vv.downcast_ref::<$ptype>() {
                           $var = fdata.clone();
                       }else{
                           panic!(format!("callback function {}'s parameter {} is not {}",$name,stringify!($var),stringify!($ptype)));
                       }
                   }else{
                       panic!(format!("callback function {}'s parameter {} is not {}",$name,stringify!($var),stringify!($ptype)));
                   }
               };)* 
                   
            $body
        }
        $Obj.RegScriptProc_P(&$name, rustcallback);  
        }
    };
}

/// Object's RetValue
/// 
/// # Example
/// star_ret!(star_parapkg!(CleGroup,"return from go",345.4));
/// ```
#[macro_export]
macro_rules! star_ret {
    ($val: expr) => {
        return Some(Box::new($val));
    }
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_call!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_call {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallObject(&$name,&[$(&$var),*]);
    };    
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_callbool!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_callbool {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallBool(&$name,&[$(&$var),*]);
    };    
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_callint!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_callint {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallInt(&$name,&[$(&$var),*]);
    };    
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_callint64!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_callint64 {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallInt64(&$name,&[$(&$var),*]);
    };    
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_callstring!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_callstring {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallString(&$name,&[$(&$var),*]);
    };    
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_calldouble!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_calldouble {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallDouble(&$name,&[$(&$var),*]);
    };    
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_callobject!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_callobject {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallObject(&$name,&[$(&$var),*]);
    };    
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_callparapkg!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_callparapkg {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallParaPkg(&$name,&[$(&$var),*]);
    };    
}

/// Call Object's Function
/// 
/// # Example
/// let retobj = star_callbinbuf!(python,"tt","hello ","world");;
/// ```
#[macro_export]
macro_rules! star_callbinbuf {
    ($Obj:ident,$name:expr, $($var: expr),*) => {
        $Obj.CallBinBuf(&$name,&[$(&$var),*]);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_get!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_get {
    ($Obj:ident,$name:expr) => {
        $Obj.Get(&$name);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_getbool!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_getbool {
    ($Obj:ident,$name:expr) => {
        $Obj.GetBool(&$name);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_getint!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_getint {
    ($Obj:ident,$name:expr) => {
        $Obj.GetInt(&$name);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_getint64!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_getint64 {
    ($Obj:ident,$name:expr) => {
        $Obj.GetInt64(&$name);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_getstring!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_getstring {
    ($Obj:ident,$name:expr) => {
        $Obj.GetString(&$name);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_getdouble!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_getdouble {
    ($Obj:ident,$name:expr) => {
        $Obj.GetDouble(&$name);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_getobject!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_getobject {
    ($Obj:ident,$name:expr) => {
        $Obj.GetObject(&$name);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_getparapkg!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_getparapkg {
    ($Obj:ident,$name:expr) => {
        $Obj.GetParaPkg(&$name);
    };    
}

/// Get Object's Value
/// 
/// # Example
/// let retobj = star_getbinbuf!(python,"tt");
/// ```
#[macro_export]
macro_rules! star_getbinbuf {
    ($Obj:ident,$name:expr) => {
        $Obj.GetBinBuf(&$name);
    };    
}

/// Set Object's Value
/// 
/// # Example
/// let retobj = star_set!(python,"g1",456);
/// ```
#[macro_export]
macro_rules! star_set {
    ($Obj:ident,$name:expr,$val:expr) => {
        $Obj.Set(&$name,&$val);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_get!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_get {
    ($Obj:ident,$name:expr) => {
        $Obj.Get($name);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_getbool!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_getbool {
    ($Obj:ident,$name:expr) => {
        $Obj.GetBool($name);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_getint!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_getint {
    ($Obj:ident,$name:expr) => {
        $Obj.GetInt($name);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_getint64!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_getint64 {
    ($Obj:ident,$name:expr) => {
        $Obj.GetInt64($name);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_getstring!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_getstring {
    ($Obj:ident,$name:expr) => {
        $Obj.GetString($name);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_getdouble!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_getdouble {
    ($Obj:ident,$name:expr) => {
        $Obj.GetDouble($name);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_getobject!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_getobject {
    ($Obj:ident,$name:expr) => {
        $Obj.GetObject($name);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_getparapkg!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_getparapkg {
    ($Obj:ident,$name:expr) => {
        $Obj.GetParaPkg($name);
    };    
}

/// Get ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_getbinbuf!(para,0);
/// ```
#[macro_export]
macro_rules! parapkg_getbinbuf {
    ($Obj:ident,$name:expr) => {
        $Obj.GetBinBuf($name);
    };    
}

/// Set ParaPkg's Value
/// 
/// # Example
/// let retobj = parapkg_set!(para,0,"2222");
/// ```
#[macro_export]
macro_rules! parapkg_set {
    ($Obj:ident,$name:expr,$val:expr) => {
        $Obj.Set($name,&$val);
    };    
}

/// RunScript
/// 
/// # Example
/// let (retobj,err) = star_runscript!("python","print(123)","","");
/// ```
#[macro_export]
macro_rules! star_runscript {
    ($CleService:ident,$interface:expr,$scriptbuf:expr,$name:expr,$path:expr) => {
        $CleService.RunScript(&$interface,&$scriptbuf,&$name,&$path);
    };    
}

/// RunScriptEx
/// 
/// # Example
/// let (retobj,err) = star_runscriptex!("python",binbuf,"","");
/// ```
#[macro_export]
macro_rules! star_runscriptex {
    ($CleService:ident,$interface:expr,$binbuf:expr,$name:expr,$path:expr) => {
        $CleService.RunScriptEx(&$interface,&$binbuf,&$name,&$path);
    };    
}

/// DoFile
/// 
/// # Example
/// let (retobj,err) = star_dofile!("python",aaa.py","");
/// ```
#[macro_export]
macro_rules! star_dofile {
    ($CleService:ident,$interface:expr,$fname:expr,$path:expr) => {
        $CleService.DoFile(&$interface,&$fname,&$path);
    };    
}

/// DoFileEx
/// 
/// # Example
/// let (retobj,err) = star_dofileex!("python",aaa.py","","");
/// ```
#[macro_export]
macro_rules! star_dofileex {
    ($CleService:ident,$interface:expr,$fname:expr,$path:expr,$mname:expr) => {
        $CleService.DoFileEx(&$interface,&$fname,&$path,&$mname);
    };    
}
