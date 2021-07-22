let Token =
      "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiIwMUY5OTJaS1lTRUhBQ1RNMVRCQ1NTNjY0MiIsImlhdCI6MTYyNjk0MTMzNywiZXhwIjoxNjI2OTQ0OTM3fQ.GBAMgQIeFwiLrIIqV5VsR7d0MrUzZtdnefNYnj-CTNI"

in  { baseUrl = "http://localhost:3030"
    , headers = toMap
        { Content-Type = "application/json"
        , Authorization = "Bearer " ++ Token
        , x-organization-id = "01F992ZM2TBXP8WZ2GKS2HCG4Z"
        }
    }
