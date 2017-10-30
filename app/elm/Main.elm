module Main exposing (main)

import Html exposing (div, h1, text)
import Html.Attributes exposing (class)


main : Html.Html msg
main =
    div [ class "jumbotron" ]
        [ h1 []
            [ text "Hello from Elm v0.18" ]
        ]
