import { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
// import "./LandingPage.css"

export default function AboutUs(props) {
   //console.log(props.user);
   const navigate = useNavigate(); 
   
   return (
     <div className = "container">
        <h1>About Us</h1>
        <h2>Team of 3 Hunter Computer Science students!</h2>
      </div>
   )
}