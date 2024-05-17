import * as React from "react";
import { Link, useNavigate } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import "./adminPanel.css"; 
import SidebarAdmin from "../SidebarAdmin/SidebarAdmin";
import adminpng from "./admin.png"; 
import { RiAdminFill } from "react-icons/ri";
import { Menu } from "antd"; 
//import { useNavigate } from "react-router-dom";
export default function AdminPanel(props) {
    //console.log(props.user);
    return (
    // <div class="adminBody">
    //   <div className="pageAdmin">
    //     <SidebarAdmin/>
    //     <div className="nameAdmin">
    //         <h1>Admin Panel</h1>
    //     </div>
    //     <div>
    //     </div>
    //   </div>
    //   </div>
    <div class="grid-container">
      <div class="appHeader">
       <h1> <RiAdminFill /> Admin Panel </h1>
      </div>
    <div className="space">
    <div class = "SideMenu"> 
    <Menu className="menu1"
    onClick={(item) => {

    }}
    items={[
        {label: "Dashboard",
        key: "/", 
     }, {
        label: "Users", 
        key: "/users", 
     }, {
        label: "Analytics", 
        key: "/analytics", 
     }, 
     {
        label: "Settings", 
        key: "/settings", 
     }, 
        ]}></Menu>
    </div>
    <div class="pageContent"> <div className="boxes"></div><div className="boxes"></div><div className="boxes"></div></div>
    <div></div>
    </div>
    </div>
    )
 }