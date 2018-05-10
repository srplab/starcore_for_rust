
## starcore_for_rust v0.5

The library is written in rust language, which supports rust language to call other scripting languages such as python, java, ruby, c#, etc.And, also support use rust language to write shared libraries for other scripting languages.

It supports android, windows and linux platforms

Change at 2018/05/10
--------

Add macro definition, fix bugs

```
star_extension!(SrvGroup,Service { ... } );

Example,

star_extension!(SrvGroup,Service {
    /*--create a new cle object, other script can find the object by it's name--*/
	let obj = Service.New(&[&"RustObject"]);
    /*--define function "PrintHello" of cle object--*/
    star_fn!(obj,"PrintHello",CleGroup,CleService,CleObject,a:String, b:f64 {
        println!("########{:?}",a);
        println!("########{:?}",b);
        star_ret!(star_parapkg!(CleGroup,"return from go",345.4));
    });     
});

```


Change at 2018/05/09
--------

Add macro definition, fix bugs

```
let (retobj,err) = star_runscript!(Service,"python","print(123)","","");
let (retobj,err) = star_runscriptex!(Service,"python",binbuf,"","");
let (retobj,err) = star_dofile!(Service,"python",aaa.py","");
let (retobj,err) = star_dofileex!(Service,"python",aaa.py","","");

let retobj = parapkg_fromvec!(para,Str,&vec!["aaa","bbb"]); // Bool,I8,U8,I16,U16,I32,U32,I64,U64,ISize,USize,F32,F64,String,Str
let retobj = parapkg_tovec!(para,String);                   // Bool,I32,I64,ISize,F64,String
```


Change at 2018/05/08
--------

Add macro definition, simplify object function definition and other operations


```
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

Example:

let CallBackObj = Service.New(&[]);

star_fn!(CallBackObj,"PrintHello",CleGroup,CleService,CleObject,a:String, b:i32 {
    println!("########{:?}",a);
    println!("########{:?}",b);
    star_ret!(star_parapkg!(CleGroup,"return from go",345.4));
});


```

Build Config:
--------

- For Build executable, Add dependencies to "Cargo.toml", and then run "cargo build"

```
[dependencies]
starcore_for_rust = {git="https://github.com/srplab/starcore_for_rust"}
```

- For Build share library, Add dependencies to "Cargo.toml", and then run "cargo build"

```
[lib]
crate-type = ["cdylib"]
path = "src/main.rs"

[dependencies]
starcore_for_rust = {git="https://github.com/srplab/starcore_for_rust",features = ["star-sharelib"]}
```

- For Build share library for android, Add dependencies to "Cargo.toml"

```
[lib]
crate-type = ["cdylib"]
path = "src/main.rs"

[dependencies]
starcore_for_rust = {git="https://github.com/srplab/starcore_for_rust",features = ["star-sharelib","android_arm"]}
```

And then,

```sh
$ rustup target add arm-linux-androideabi
# Create toolchain
$ /home/xxx/Android/android-ndk-r12b/build/tools/make-standalone-toolchain.sh --platform=android-18 --toolchain=arm-linux-androideabi-4.9 --install-dir=android-18-toolchain --ndk-dir=/home/lihm/Android/android-ndk-r12b / --arch=arm
# Create the .cargo/config file
$ mkdir -p .cargo
$ echo '[build]
target = "arm-linux-androideabi"

[target.arm-linux-androideabi]
linker = "/home/xxx/Desktop/rust.study/android-18-toolchain"' > .cargo/config
$ cargo build
```

Example:
--------

```rust
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate starcore_for_rust;
use starcore_for_rust::*;

fn main() {
	let Service = starrust::InitSimple(&"test",&"123", 0, 0,&[]);
	let SrvGroup = Service.Get(&"_ServiceGroup").ToSrvGroup();
        
    let initResult = SrvGroup.InitRaw(&"python36", &Service);
    let python = Service.ImportRawContext(&"python", &"", false, &"");

	python.Call(&"import", &[&"sys"]);
	let python_sys = python.GetObject(&"sys");
	let python_python = python_sys.GetObject(&"path");
	println!("{}",python_python.ToString());
}
```

Capture output from other scripts:
--------

Register CallBack function, as follow

```rust
extern crate starcore_for_rust;
use starcore_for_rust::*;
use std::any::Any;

fn MsgCallBack(ServiceGroupID: u32, uMsg: u32, wParam: &Any, lParam: &Any) -> (bool, Box<Any>)
{
	if uMsg == MSG_VSDISPMSG || uMsg == MSG_VSDISPLUAMSG || uMsg == MSG_DISPMSG || uMsg == MSG_DISPLUAMSG {
		println!("{}",starrust::ToString(wParam));
	} else {
	}
	return (false, Box::new(&0));
}

fn main() {
	let Service = starrust::InitSimple(&"test",&"123", 0, 0,&[]);
    starrust::RegMsgCallBack_P(MsgCallBack);
	let SrvGroup = Service.Get(&"_ServiceGroup").ToSrvGroup();

    ...
}
```

Develop extension modules:
--------

- Create and export "StarCoreService_Init2" and "StarCoreService_Term2" function, as follow,

```rust
#[no_mangle]
pub extern "C" fn StarCoreService_Init2(StarCore:*const c_void, InterfaceTable:*const c_void ) -> i8
{
    starrust::println(format!("{}","StarCoreService_Init2"));
    let res = starrust::Stub_StarCoreService_Init2(StarCore,InterfaceTable);
    starrust::println(format!("{}",res));
    return res;
}
    
#[no_mangle]
pub extern "C" fn StarCoreService_Term2(StarCore:*const c_void, InterfaceTable:*const c_void )
{
    starrust::println(format!("{}","StarCoreService_Term2"));
    starrust::Stub_StarCoreService_Term2(StarCore,InterfaceTable);
}
```

- Create and export "ScriptInitCallBack" and "ScriptTermCallBack" function. In ScriptInitCallBack, create cle objects which will be output to other scripts

```rust
fn rustcallback(CleGroup:&STARSRVGROUP,CleService:&STARSERVICE,CleObject:&starrust::STAROBJECT,Paras: &[starrust::STARRESULT]) -> starrust::STARRESULT {
    starrust::println(format!("{}",Paras[0].ToString()));
    starrust::println(format!("{}",Paras[1].ToInt()));
	return Some(Box::new(CleGroup.NewParaPkg(&[&"return from rust", &345.4])));
}
#[no_mangle]
pub extern "C" fn ScriptTermCallBack() {}
#[no_mangle]
pub extern "C" fn ScriptInitCallBack(SrvGroup: &STARSRVGROUP, Service: &STARSERVICE) {
    starrust::println(format!("{}",SrvGroup.ToString()));
    starrust::println(format!("{}",Service.ToString()));

	let s = Service.New(&[&"RustObject"]);
	s.RegScriptProc_P(&"PrintHello", rustcallback);    
}
```

### Build to share library

See above


### Use the share library, 

use "_DoFile" function of cle to load the share library,  
example on android


```java
package com.example.srplab.testrust;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;

import com.srplab.www.starcore.*;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        /*----init starcore----*/
        StarCoreFactoryPath.StarCoreCoreLibraryPath = this.getApplicationInfo().nativeLibraryDir;
        StarCoreFactoryPath.StarCoreShareLibraryPath = this.getApplicationInfo().nativeLibraryDir;
        StarCoreFactoryPath.StarCoreOperationPath = "/data/data/"+getPackageName()+"/files";

        final StarCoreFactory starcore= StarCoreFactory.GetFactory();
        StarServiceClass Service=starcore._InitSimple("test","123",0,0);
        starcore._RegMsgCallBack_P(new StarMsgCallBackInterface() {
                                       public Object Invoke(int ServiceGroupID, int uMes, Object wParam, Object lParam) {
                                           if (uMes == starcore._GetInt("MSG_VSDISPMSG") || uMes == starcore._GetInt("MSG_VSDISPLUAMSG")) {
                                               System.out.println((String) wParam);
                                           }
                                           if (uMes == starcore._GetInt("MSG_DISPMSG") || uMes == starcore._GetInt("MSG_DISPLUAMSG")) {
                                               System.out.println("++++++++++++++++" + (String) wParam);
                                           }
                                           return null;
                                       }
                                   });
        StarSrvGroupClass SrvGroup = (StarSrvGroupClass)Service._Get("_ServiceGroup");
        Service._CheckPassword(false);

        Object[] result = Service._DoFile("",this.getApplicationInfo().nativeLibraryDir+"/libteststarrust.so","");
        System.out.println(result);

        System.out.println(Service._Get("RustObject"));
        StarObjectClass RustObject = (StarObjectClass)Service._GetObject("RustObject");
        System.out.println(RustObject);
        System.out.println(RustObject._Call("PrintHello","------------1",234.56));
    }
}
```


More Info:
--------

For more information about use how to use cle's APIs, please refer to 

[common language extension_programmer's guide_en.pdf](https://github.com/srplab/starcore_for_android/blob/master/docs/common%20language%20extension_programmer's%20guide_en.pdf)

and,

[common language extension_interface for script_en.pdf](https://github.com/srplab/starcore_for_android/blob/master/docs/common%20language%20extension_interface%20for%20script_en.pdf)

For more examples, please refer to 

[srplab.github.io](https://srplab.github.io)
