module View exposing (..)

import Views.HeaderView exposing (navBar, flashMessage)
import Html exposing (Html, Attribute, button, div, fieldset, form, h2, h3, input, li, text, ul)
import Html.Attributes exposing (attribute, class, id, type_, name, placeholder, value)
import Html.Events exposing (onClick)
import Models exposing (Model, Flash)
import Msgs exposing (Msg)


-- TODO: Animation on formField


view : Model -> Html Msg
view model =
    div [ class "container-fluid" ]
        [ navBar model
        , div [ class "container" ]
            [ flashMessage model.flash
            , page model
            ]
        ]


page : Model -> Html Msg
page model =
    case model.route of
        Models.IndexPage ->
            indexView model

        Models.SignupPage ->
            signupView model

        Models.NotFoundPage ->
            notFoundView


indexView : Model -> Html Msg
indexView model =
    div []
        [ text "Welcome to Elm and Rocket" ]


signupView : Model -> Html Msg
signupView model =
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

                    -- fieldset [ personal details ]
                    , fieldset []
                        [ h2 [ class "fs-title" ]
                            [ text "Personal Details" ]
                        , h3 [ class "fs-subtitle" ]
                            [ text "Please provide info" ]
                        , input [ type_ "text", name "fname", placeholder "First Name" ]
                            []
                        , input [ type_ "text", name "lname", placeholder "Last Name" ]
                            []
                        , input [ type_ "text", name "email", placeholder "Email" ]
                            []
                        , input
                            [ type_ "button"

                            -- , onClick Msgs.ProfileNext
                            , name "next"
                            , class "next action-button"
                            , value "Next"
                            ]
                            []
                        ]

                    -- fieldset [ social profile ]
                    , fieldset []
                        [ h2 [ class "fs-title" ]
                            [ text "Social Profile" ]
                        , h3 [ class "fs-subtitle" ]
                            [ text "Please provide a social network profile" ]
                        , input [ type_ "text", name "twitter", placeholder "Twitter" ]
                            []
                        , input [ type_ "text", name "facebook", placeholder "Facebook" ]
                            []
                        , input [ type_ "text", name "instagram", placeholder "Instagram" ]
                            []
                        , input
                            [ type_ "button"

                            -- , onClick Msgs.SocialPrevious
                            , name "previous"
                            , class "previous action-button-previous"
                            , value "Previous"
                            ]
                            []
                        , input
                            [ type_ "button"

                            -- , onClick Msgs.SocialNext
                            , name "next"
                            , class "next action-button"
                            , value "Next"
                            ]
                            []
                        ]

                    -- fieldset [ create account ]
                    , fieldset []
                        [ h2 [ class "fs-title" ]
                            [ text "Create a new account" ]
                        , h3 [ class "fs-subtitle" ]
                            [ text "Please provide new account details" ]
                        , input [ type_ "text", name "username", placeholder "username" ]
                            []
                        , input [ type_ "password", name "pwd", placeholder "Password" ]
                            []
                        , input [ type_ "password", name "cpwd", placeholder "Confirm Password" ]
                            []
                        , input
                            [ type_ "button"

                            -- , onClick Msgs.CredentialsPrevious
                            , name "previous"
                            , class "previous action-button-previous"
                            , value "Previous"
                            ]
                            []
                        , input
                            [ type_ "button"
                            , onClick Msgs.SignupSubmit
                            , name "submit"
                            , class "submit action-button"
                            , value "Submit"
                            ]
                            []
                        ]
                    ]
                ]
            ]
        ]


notFoundView : Html msg
notFoundView =
    div []
        [ text "Not found"
        ]


for : String -> Attribute msg
for value =
    attribute "for" value
