module Commands exposing (..)

import Http
import Json.Decode as Decode
import Json.Decode exposing (field)
import Json.Encode as Encode
import Models exposing (FormFields, Session, User)
import Msgs exposing (Msg)
import RemoteData


-- TODO: sessions, login request, login url


signupUser : FormFields -> Cmd Msg
signupUser formFields =
    signupRequest formFields
        |> RemoteData.sendRequest
        |> Cmd.map Msgs.OnFetchSession


signupRequest : FormFields -> Http.Request Session
signupRequest formFields =
    Http.request
        { body = encodeSession formFields |> Http.jsonBody
        , expect = Http.expectJson decodeSession
        , headers = []
        , method = "POST"
        , timeout = Nothing
        , url = signupUrl
        , withCredentials = False
        }


signupUrl : String
signupUrl =
    "http://localhost:4000/api/v1/register"


decodeSession : Decode.Decoder Session
decodeSession =
    Decode.map2 Session
        (field "user" decodeUser)
        (field "jwt" Decode.string)


decodeUser : Decode.Decoder User
decodeUser =
    Decode.map3 User
        (field "id" Decode.int)
        (field "name" Decode.string)
        (field "email" Decode.string)


encodeSession : FormFields -> Encode.Value
encodeSession formFields =
    Encode.object
        [ ( "user", encodeUser <| formFields )
        ]


encodeUser : FormFields -> Encode.Value
encodeUser record =
    Encode.object
        [ ( "name", Encode.string <| record.username )
        , ( "email", Encode.string <| record.email )
        , ( "password", Encode.string <| record.pass )
        ]
