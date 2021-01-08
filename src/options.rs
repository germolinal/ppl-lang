
pub type Options<T> = Vec<T>;


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;    
    use crate::compiler::CompilerOptions;
    
    

    #[test]
    fn test_compiler_options(){
        let mut o : Options<CompilerOptions> = vec![];

        assert!( !o.contains( &CompilerOptions::Optimize ));

        o.push(CompilerOptions::Optimize);
        assert!( o.contains( &CompilerOptions::Optimize ));
    }

}