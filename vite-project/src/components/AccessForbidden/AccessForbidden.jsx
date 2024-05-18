import * as React from "react"
import "./AccessForbidden.css"

export default function AccessForbidden() {
  return (
    <div className="access-forbidden">
        <div className="content">
           <h1 className ="access-h1">You can't view this page.</h1>
           <p className = "access-h1"><a href="/Login">Log In </a>or <a href="/Register">Register </a>to view this page.</p>
        </div>
    </div>
  )
}