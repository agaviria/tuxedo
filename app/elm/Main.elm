module Main exposing (main)

import Html exposing (button, div, fieldset, form, h2, h3, input, li, text, ul)
import Html.Attributes exposing (class, id, type_, name, placeholder, value)


main : Html.Html msg
main =
    div [ class "container" ]
        [ div [ class "row" ]
            [ div [ class "col-md-6 col-md-offset-3" ]
                [ div [ id "msform" ]
                    -- progressbar
                    [ ul [ id "progressbar" ]
                        [ li
                            [ class "active" ]
                            [ text "Personal Details" ]
                        , li [] [ text "Social Profile" ]
                        , li [] [ text "Account Setup" ]
                        ]

                    -- fieldsets
                    , fieldset []
                        [ h2 [ class "fs-title" ]
                            [ text "Personal Details" ]
                        ]

                    -- , fieldset []
                    --     [ h3 [ class "fs-subtitle" ]
                    --         [ text "Please provide info" ]
                    --     ]
                    -- , fieldset []
                    --     [ input [ type_ "text", name "fname", placeholder "First Name" ]
                    --         []
                    --     ]
                    -- , fieldset []
                    --     [ input [ type_ "text", name "lname", placeholder "Last Name" ]
                    --         []
                    --     ]
                    -- , fieldset []
                    --     [ input [ type_ "text", name "email", placeholder "Email" ]
                    --         []
                    --     ]
                    -- , fieldset []
                    --     [ input [ type_ "button", class "next action-button", name "Next", value "Next" ]
                    --         []
                    --     ]
                    ]
                ]
            ]
        ]
