pub const CONTROLLER: &'static str = r#"
       identification division.
       program-id. __SCRIPT_NAME__.

       data division.
       working-storage section.

       01 the-vars.

          03  COW-vars OCCURS 99 times.

            05 COW-varname       pic x(99).
            05 COW-varvalue      pic x(99).

       linkage section.

       01 the-values.

          05 COW-query-values           occurs 10 times.
            10 COW-query-value-name     pic x(90).
            10 COW-query-value          pic x(90).


       procedure division using the-values.


           __VARIABLES__

           call 'cowtemplate' using the-vars "__SCRIPT_NAME__.cow".


       goback.

       end program showname.
"#;


pub const VARIABLE: &'static str = r#"
           MOVE "__NAME__" to COW-varname(__INDEX__).
           MOVE query-value(__INDEX__) to COW-varvalue(__INDEX__).
"#;


/*
move 4 to nroutes.

move "/"                           to routing-pattern(1).
move "indexweb"                    to routing-destiny(1).

move "/showsum/%value1/%value2"    to routing-pattern(2).
move "showsum"                     to routing-destiny(2).

move "/showname/%value"            to routing-pattern(3).
move "showname"                    to routing-destiny(3).

move "/anotherroute"               to routing-pattern(4).
move "anotherroute"                to routing-destiny(4).

*/
pub const ROUTES: &'static str = r#"
move __ROUTE_COUNT__ to nroutes.

__ROUTES__
"#;

pub const ROUTE: &'static str = r#"
           move "__PATTERN__" to routing-pattern(__INDEX__).
           move "__DESTINY__" to routing-destiny(__INDEX__).
"#;

pub const VIEW: &'static str = r#"
<html>
    <head>
        __TITLE__
        <script src="https://cdnjs.cloudflare.com/ajax/libs/alpinejs/2.3.0/alpine-ie11.js"> </script>
        <script>
            __VARIABLES__
        </script>
    </head>
    <body>
        __BODY__
    </body>
</html>
"#;

pub const VIEW_VARIABLES: &'static str = r#"
        window.__NAME__ = {{__NAME__}};
"#;



