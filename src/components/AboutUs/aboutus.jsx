import { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
// import "./LandingPage.css"
import "./aboutus.css";
export default function AboutUs(props) {
   //console.log(props.user);
   const navigate = useNavigate(); 
   
   return (
     <div className = "container-about" style={{ fontFamily: 'Rubik, sans-serif' }}>
        <h1>About Us</h1>
        <h2>Team of 3 Hunter Computer Science students!</h2>
        <div className="avatar">
      <div className="w-24 rounded-full">
      <img src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" />
      </div>
      <div className="w-24 rounded-full">
      <img src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" />
      </div>
      <div className="w-24 rounded-full">
      <img src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" />
      </div>
      </div>
      </div>
   )
}