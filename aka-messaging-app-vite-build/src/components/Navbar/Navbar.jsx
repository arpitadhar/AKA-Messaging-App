import * as React from "react"
import { useState, useEffect } from "react"
import { Link , useNavigate} from "react-router-dom"
import "./Navbar.css"

export default function Navbar(props) {

    const navigate = useNavigate();

    const clickHandler = (event) => {
        props.setUser(false);
        navigate("/");
    }

    return (
        <nav class="navbar navbar-dark bg-dark">
            <a class="navbar-brand text-white px-3" href="/">AKA Messaging</a>
            
            <div class="px-3">
                {props.user ? 

                    <div>
                        <Link id="log-link" to="/user">
                            <button class="btn btn-outline-success my-2 my-sm-0"  type="button">Profile</button>
                        </Link>
                        sd
                        <button class="btn btn-success my-2 my-sm-0"  type="button" onClick={clickHandler}>Logout</button>

                    </div>
                :
                    <div>
                    <Link id="log-link" to="/login">
                        <button class="btn btn-outline-success my-2 my-sm-0"  type="button">Login</button>
                    </Link>
                    sd
                    <a href="/register">
                        <button class="btn btn-success my-2 my-sm-0"  type="button">Register</button>
                    </a>
                    </div>
                }
            </div>

        </nav>
    )
}