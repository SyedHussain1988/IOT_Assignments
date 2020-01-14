pub mod lib_telecom
{


    #[derive(Debug)]
    pub struct OperatorDetailsLib
    {
        pub name:String,
        pub spectrum:String
    }

    pub mod lib_telecom_company_4g
    {
        
        pub fn lib_operator()
        {
            println!("lib.rs        4G Operators are Mobilink, Zong, Telenor");
        }

    }

    pub mod lib_telecom_company_3g
    {
        
       pub fn lib_operator()
        {
            println!("lib.rs        3G Operator is Ufone");
        }

    }


}