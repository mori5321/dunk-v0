let base = ./base.dhall

let req =
          base
      //  { path = "/auth/login"
          , method = "POST"
          , reqBody = Some
              ( toMap
                  { logIn = { email = "mossss@hoge.com", password = "password" }
                  }
              )
          }

in  req
