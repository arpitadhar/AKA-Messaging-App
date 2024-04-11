import * as React from "react"
import "./AccessForbidden.css"

export default function AccessForbidden() {
  return (
    <div className="access-forbidden">
        <div className="content">
           <h1>You can't view this page.</h1>
           <p>Log In or Register to view this page.</p>
        </div>
    </div>
  )
}