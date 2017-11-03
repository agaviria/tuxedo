module Msgs exposing (..)

import Navigation exposing (Location)
import RemoteData exposing (WebData)
import Models exposing (User, Session)


-- TODO:  Look into SingupSubmit and underpinnings


type Msg
    = OnLocationChange Location
    | OnFetchSession (WebData Session)
    | Logout
      -- | ProfileNext
      -- | SocialPrevious
      -- | SocialNext
      -- | CredentialsPrevious
    | SignupSubmit
