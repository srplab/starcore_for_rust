# Common Language Extension (CLE) 

Portability is an issue we must consider when developing applications. The most portable language is ansi c. However, due to the lack of functional libraries, applications are not easy to write and need to take a long time to market. An alternative is to use a scripting language, such as python, ruby, or lua. With the help of CLE middleware, these scripting languages can run on most platforms such as windows, windows 10, ios, android, linux.

CLE is a middleware to support mixed programming of multiple languages. Any language of c/c++, lua, python, c#, java can access most classes, functions, variables, or modules of other languages directly, which makes existing code or libraries to be reused easily. Developers can write code using the favorite language and then use them in other applications written with different languages. CLE is simple, which is a single core share library with libraries corresponding to each script language.

We recommend that developers write primary logic in a scripting language, and GUI or device-specific parts in the platform-dependent language. Not only ensure application portability, but also take advantage of the platform SDK. Using scripting language may result in a little large about the size of installation packages and a slightly decrease in performance, but these should not be a problem as hardware performance improves and storage increases
 

### It's major features are listed below:

- Supporting multiple platforms. CLE supports windows xp, windows 7, windows 8, linux x86, android, ios, windows 10
- Supporting multiple script languages. CLE supports c/c++, lua, python, java, c#,ruby
- Integrated two-way bridge between scripts. Any language of the c/c++, lua, python, java, c# can access most classes, functions, methods, or modules of another language directly.
- Providing unified interface to multiple script language.


## starcore_for_rust v0.5

The library is written in rust language, which supports rust language to call other scripting languages such as python, java, ruby, c#, etc.And, also support use rust language to write shared libraries for other scripting languages.

It supports android, windows and linux platforms

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
