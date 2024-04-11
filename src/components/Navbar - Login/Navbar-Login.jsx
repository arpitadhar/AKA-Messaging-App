import * as React from "react"
import { useState, useEffect } from "react"
import { Link , useNavigate} from "react-router-dom"
import "./Navbar-Login.css"

export default function Navbar(props) {
    console.log(props.user);
    const clickHandler = (event) => {
        console.log(props.user);
        props.setUser(false);
    }

    return (
        <nav class="navbar navbar-dark bg-transparent">
            <a class="navbar-brand text-white px-3" href="/">AKA Messaging</a>
            
            <div class="px-3">
                {props.user ? 

                    <button class="btn btn-success my-2 my-sm-0"  type="button" onClick={clickHandler}>Logout</button>

                :
                    <div>
                    </div>
                }
            </div>

        </nav>
    )
}