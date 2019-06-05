
use deco::*;


fn main() {
    tcprint!(
        [
        "Foreground\n"
        black "black\n"
        red "red\n"
        green "green\n"
        yellow "yellow\n"
        blue "blue\n"
        magenta "magenta\n"
        cyan "cyan\n"
        white "white\n"

        reset

        "Background\n"
        Black "black" ! "\n"
        Red "red" ! "\n"
        Green "green" ! "\n"
        Yellow "yellow" ! "\n"
        Blue "blue" ! "\n"
        Magenta "magenta" ! "\n"
        Cyan "cyan" ! "\n"
        White "white" ! "\n"

        ! // == `reset`

        "Decoration\n"
        bold "bold\n" !
        italic "italic\n" !
        underscore "underscore\n" !
        blink "blink\n" !
        reverse "reverse\n"  !
        conceal "conceal\n" !
        ]
    );
}
