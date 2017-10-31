module Main exposing (main)

import Html exposing (div, form, h1, li, text, ul)
import Html.Attributes exposing (class, id)


main : Html.Html msg
main =
    div [ class "container" ]
        [ div [ class "row" ]
            [ div [ class "col-md-6 col-md-offset-3" ]
                [ div [ id "msform" ]
                    [ ul [ id "progressbar" ]
                        [ li
                            [ class "active" ]
                            [ text "Personal Details" ]
                        ]
                    , li [] [ text "Social Profile" ]
                    , li [] [ text "Account Setup" ]
                    ]
                ]
            ]
        ]
