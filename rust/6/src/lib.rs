pub enum Operator {
    // `+`
    Add, 
    // `-`
    Sub,
    // `*`
    Mul,
}

pub enum Token {
    Operator(Operator),
    Operand(isize),
}

/// Evaluates the postix expression.
///
/// Input: a postfix expression, where each element contains an operator or operand.
/// Returns: if the postfix expression is valid, returns `Some(value)`;
///     otherwise, returns `None`.
pub fn eval(tokens: &[Token]) -> Option<isize> {
	let mut stack = vec![] ;
	for element in  tokens{
    	match element {
    		&Token ::Operand(operand) => {
    			let token = operand;
    			stack.push(token)
    		}
    		&Token ::Operator(ref operator) =>  {
    			let  element1 = stack.pop();
    			let  element2 = stack.pop();
    			if element1!=None|| element2!=None {
    				match operator {
    					&Operator :: Add =>element1.unwrap() + element2.unwrap(),
    					&Operator :: Sub =>element1.unwrap() - element2.unwrap(),
    					&Operator :: Mul =>element1.unwrap() * element2.unwrap(),
    				};
    			}else {
    				return None
    			}

    				
    			}
    			
    		}
        }
     if stack.len() > 1  {
     	None
     } else  {
     	Some(stack[0])
     }

    			
    	}

fn main(){
    
}

    	


    	

    			
    			
    

    		 
			


    	
    
    
