let base = ./base.dhall

let req =
          base
      //  { path = "/customers/returnForms/01FACFZPTXZA7QF1NEB21NBAS3"
          , method = "GET"
          , reqBody = None Text
          }

in  req
