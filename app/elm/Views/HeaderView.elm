module Views.HeaderView exposing (..)

import Html exposing (..)
import Html.Events exposing (onClick)
import Html.Attributes exposing (..)
import Models exposing (Model, User, Flash)
import Msgs exposing (Msg)
import Util exposing (styles)
import Css exposing (backgroundColor, hex, padding, px)


cComfort : Css.Color
cComfort =
    hex "EEEEFF"


navBar : Model -> Html Msg
navBar model =
    nav
        [ class "navbar navbar-expand navbar-light bd-navbar"
        , styles
            [ backgroundColor cComfort
            , padding (px 20)
            ]
        ]
        [ a [ class "navbar-brand", href "#" ] [ text "home" ]
        , navBarCollapseButton
        , div [ class "collapse navbar-collapse", id "navbarSupportedContent" ]
            [ ul [ class "navbar-nav mr-auto" ]
                [ li [ class "nav-item active" ] [ currentUserNav model.currentUser ] ]
            ]
        ]


navBarCollapseButton : Html Msg
navBarCollapseButton =
    button
        [ class "navbar-toggler navbar-toggler-right"
        , attribute "type" "button"
        , attribute "data-toggle" "collapse"
        , attribute "data-target" "#navbarSupportedContent"
        , attribute "aria-controls" "navbarSupportedContent"
        , attribute "aria-expanded" "false"
        , attribute "aria-label" "Toggle navigation"
        ]
        [ span [ class "navbar-toggler-icon" ] [] ]


currentUserNav : Maybe User -> Html Msg
currentUserNav currentUser =
    case currentUser of
        Just user ->
            a [ onClick Msgs.Logout, class "nav-link", href "#login" ] [ text "Logout" ]

        Nothing ->
            a [ href "#signup", class "nav-link" ]
                [ text "Sign up" ]


flashMessage : Maybe Flash -> Html Msg
flashMessage flash =
    case flash of
        Just flash ->
            flashAlert flash

        Nothing ->
            text ""


flashAlert : Flash -> Html Msg
flashAlert flash =
    case flash.flash_type of
        "error" ->
            div [ class "alert alert-danger" ] [ text flash.message ]

        other ->
            div [ class "alert alert-primary" ] [ text flash.message ]
