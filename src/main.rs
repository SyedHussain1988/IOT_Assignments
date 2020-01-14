mod telecom
{


    #[derive(Debug)]
    pub struct OperatorDetails
    {
        pub name:String,
        pub spectrum:String
    }

    pub mod telecom_company_4g
    {
        
        pub fn operator()
        {
            println!("main.rs        4G Operators are Mobilink, Zong, Telenor");
        }

    }

    pub mod telecom_company_3g
    {
        
       pub fn operator()
        {
            println!("main.rs        3G Operator is Ufone");
        }

    }


}


mod lib;//same environmentlib.rs
use libraryHelper;//cargo.toml libraryHelper added
use crate::lib::lib_telecom::lib_telecom_company_4g;
use crate::lib::lib_telecom::lib_telecom_company_3g;

use crate::telecom::telecom_company_4g;
use crate::telecom::telecom_company_3g;

use crate::libraryHelper::lib_ext_telecom::lib_ext_telecom_company_4g;
use crate::libraryHelper::lib_ext_telecom::lib_ext_telecom_company_3g;

fn main() {
    
    println!("\n--------------------Module, Sub Module from main START-------------------------------\n");
    crate::telecom::telecom_company_4g::operator();//absolute path
    crate::telecom::telecom_company_3g::operator();//absolute path
    println!("---------------------------------------------------");
    telecom_company_4g::operator();//absolute path
    telecom_company_3g::operator();//absolute path
    println!("---------------------------------------------------");
    let telecom_data=telecom::OperatorDetails
    {
    name:"Ufone".to_string(),
    spectrum:"3G".to_string()
    };

    println!("{:#?}",telecom_data);
    println!("\n--------------------Module, Sub Module from main END-------------------------------\n");




    println!("\n--------------------Module, Sub Module from internal library START-------------------------------\n");
    crate::lib::lib_telecom::lib_telecom_company_4g::lib_operator();//absolute path
    crate::lib::lib_telecom::lib_telecom_company_3g::lib_operator();//absolute path
    println!("---------------------------------------------------");
    lib_telecom_company_4g::lib_operator();//absolute path
    lib_telecom_company_3g::lib_operator();//absolute path
    println!("---------------------------------------------------");
    let telecom_data_lib=lib::lib_telecom::OperatorDetailsLib
    {
    name:"Ufone".to_string(),
    spectrum:"3G".to_string()
    };

    println!("{:#?}",telecom_data_lib);
    println!("\n--------------------Module, Sub Module in internal library END-------------------------------\n");


    println!("\n--------------------Module, Sub Module from external library START-------------------------------\n");
    
    crate::libraryHelper::lib_ext_telecom::lib_ext_telecom_company_3g::lib_ext_operator();//absolute path
    crate::libraryHelper::lib_ext_telecom::lib_ext_telecom_company_4g::lib_ext_operator();//absolute path
    println!("---------------------------------------------------");
    lib_ext_telecom_company_4g::lib_ext_operator();//absolute path
    lib_ext_telecom_company_3g::lib_ext_operator();//absolute path
    println!("---------------------------------------------------");
    let telecom_data_lib_ext=libraryHelper::lib_ext_telecom::OperatorDetailsLibExt
    {
    name:"Ufone".to_string(),
    spectrum:"3G".to_string()
    };

    println!("{:#?}",telecom_data_lib_ext);

    println!("\n--------------------Module, Sub Module from external library END-------------------------------\n");












}
