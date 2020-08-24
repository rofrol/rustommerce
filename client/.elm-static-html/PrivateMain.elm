
port module PrivateMain exposing (..)

import Platform
import Html exposing (Html)
import ElmHtml.InternalTypes exposing (decodeElmHtml)
import ElmHtml.ToString exposing (nodeToStringWithOptions, defaultFormatOptions)
import Json.Decode as Json
import Native.Jsonify

import Main


asJsonString : Html msg -> String
asJsonString = Native.Jsonify.stringify

options = { defaultFormatOptions | newLines = True, indent = 4 }

decode : (String, Html msg) -> ( String, String )
decode (output, view) =
    case Json.decodeString decodeElmHtml (asJsonString view) of
        Err str -> (output, str)
        Ok str -> (output, nodeToStringWithOptions options str)

main = Platform.program
    { init =
        ( ()
        , htmlOut ( List.map (decode ) [ ("dist/body-static.html", Main.initialView) ] )
        )
    , update = (\_ b -> (b, Cmd.none))
    , subscriptions = (\_ -> Sub.none)
    }

port htmlOut : List (String, String) -> Cmd msg
