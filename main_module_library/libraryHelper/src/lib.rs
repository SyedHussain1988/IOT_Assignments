pub mod lib_ext_telecom
{


    #[derive(Debug)]
    pub struct OperatorDetailsLibExt
    {
        pub name:String,
        pub spectrum:String
    }

    pub mod lib_ext_telecom_company_4g
    {
        
        pub fn lib_ext_operator()
        {
            println!("library helper src lib.rs        4G Operators are Mobilink, Zong, Telenor");
        }

    }

    pub mod lib_ext_telecom_company_3g
    {
        
       pub fn lib_ext_operator()
        {
            println!("library helper src lib.rs        3G Operator is Ufone");
        }

    }


}