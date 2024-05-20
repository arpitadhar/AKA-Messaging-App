import { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
// import "./LandingPage.css"
import "./aboutus.css";
import { Carousel } from 'antd';
const contentStyle = {
  height: '560px',
  color: '#fff',
  lineHeight: '160px',
  textAlign: 'center',
  background: '#00',
};
export default function AboutUs(props) {
   //console.log(props.user);
   const navigate = useNavigate(); 
   
   return (
      <div className = "container-about" style={{ fontFamily: 'Rubik, sans-serif' }}>
      <h1>About Us</h1>
      <Carousel autoplay>
      <div>
        <h3 style={contentStyle}>Arpita Dhar</h3>
      </div>
      <div>
        <h3 style={contentStyle}>Anthony Garcia</h3>
        
      </div>
      <div>
        <h3 style={contentStyle}>Kevin Gomes</h3>
      </div>
      </Carousel>
      </div>
   )
}

