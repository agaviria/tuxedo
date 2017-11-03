module Models exposing (..)

-- MODEL


type alias Model =
    { currentUser : Maybe User
    , jwt : Maybe String
    , route : Route
    , formFields : FormFields
    , flash : Maybe Flash
    }


initModel : Route -> Model
initModel route =
    { currentUser = Nothing
    , jwt = Nothing
    , route = route
    , formFields = initFormFields
    , flash = Nothing
    }


initFormFields : FormFields
initFormFields =
    { fname = ""
    , lname = ""
    , email = ""
    , facebook = ""
    , instagram = ""
    , gplus = ""
    , username = ""
    , pass = ""
    , cpass = ""
    }


type alias FormFields =
    { fname : String
    , lname : String
    , email : String
    , facebook : String
    , instagram : String
    , gplus : String
    , username : String
    , pass : String
    , cpass : String
    }


type alias User =
    { id : Int
    , username : String
    , email : String
    }


type alias Session =
    { user : User
    , jwt : String
    }


type Route
    = IndexPage
    | SignupPage
    | NotFoundPage


type alias Flash =
    { message : String
    , flash_type : String
    }
