# Gramma
***main*** -> function main() ***block***

***func*** -> function ***id*** ***params*** ***block***

***statem***

***block*** -> { ***statem*** }



***call*** -> ***id***(***params***);

***params*** -> ***param*** | ε

***param*** -> ***param***, ***par*** | ***par***

***par*** -> *id*


*id* -> ([ _ | a-z | A-Z ][ a-z | A-Z | 0-9 | _ ]\*)

*digit* ->  0-9  


## if
if (expr) block | instr

## if-else
if (expr) block | instr else block | instr

## function
function id() block




    function f() {

        x = 1                   LOAD_VAL 1
                                WRITE_VAR ‘x’

        y = 2                   LOAD_VAL 2
                                WRITE_VAR ‘y’

        return (x + 1) * y      READ_VAR ‘x’
                                LOAD_VAL 1
                                ADD

                                READ_VAR ‘y’
                                MULTIPLY

                                RETURN_VALUE
    }


    function main() {
        f();
    }

# Keywords and tokens
function
(
)
=
{
}
return
+
* 
- 
/ 

