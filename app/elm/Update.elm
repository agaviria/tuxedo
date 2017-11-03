module Update exposing (..)

import Commands exposing (..)
import Msgs exposing (Msg)
import Models exposing (Model, Session)
import RemoteData exposing (WebData)
import Routing exposing (parseLocation)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Msgs.OnLocationChange location ->
            let
                newRoute =
                    Routing.parseLocation location

                clearedModel =
                    model |> clearFields
            in
                ( { clearedModel | route = newRoute }, Cmd.none )

        Msgs.Logout ->
            ( model |> logout, Cmd.none )

        -- Msgs.ProfileNext ->
        --     ( model |> updateProfile, signupUser model.formFields )
        Msgs.SignupSubmit ->
            ( model |> clearFields, signupUser model.formFields )

        Msgs.OnFetchSession session ->
            ( model |> addSession session, Cmd.none )


logout : Model -> Model
logout model =
    { model | currentUser = Nothing, jwt = Nothing }


addSession : WebData Session -> Model -> Model
addSession sessionData model =
    case sessionData of
        RemoteData.Success session ->
            { model | currentUser = Just session.user, jwt = Just session.jwt }

        RemoteData.Failure error ->
            model |> addFlashError (toString error)

        _ ->
            model


addFlashError : String -> Model -> Model
addFlashError message model =
    let
        newFlash =
            { message = message
            , flash_type = "error"
            }
    in
        { model | flash = Just newFlash }


clearFields : Model -> Model
clearFields model =
    { model | formFields = Models.initFormFields }
