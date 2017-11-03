module Routing exposing (..)

import Navigation exposing (Location)
import Models exposing (Route(..))
import UrlParser exposing (..)


parseLocation : Location -> Route
parseLocation location =
    case (parseHash matchers location) of
        Just route ->
            route

        Nothing ->
            NotFoundPage


matchers : Parser (Route -> a) a
matchers =
    oneOf
        [ map IndexPage top
        , map SignupPage (s "signup")
        ]
