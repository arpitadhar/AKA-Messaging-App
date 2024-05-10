import * as React from "react";
import { useEffect, useState } from "react";
import { Link, Navigate, useNavigate } from "react-router-dom";
import AccessForbidden from "../AccessForbidden/AccessForbidden";
import "./Admin.css";




class Card extends React.Component {   

    clickHandler = ( event) => {
        if (window.confirm("You are about to remove this user. \n Are you sure you want to take this action? ") == true) {
            this.props.setDeleteElement(true);
          }
    }
    
    render() {
        return (
            <div> 
                <div class="row" id={this.props.id} onClick={this.clickHandler}>
                    <div class="col ">
                        <h4>{this.props.user}</h4>
                    </div>
                    <div class="col ">
                        <h4>{this.props.email}</h4>
                    </div>
                    <div class="col ">
                        <h4>{this.props.name}</h4>
                    </div>
                </div>
            </div>
        );
    }
}

class Reports extends React.Component {   
    render() {
        console.log(this.props)
        return (
            <div> 
                <div class="row" id={this.props.id}>
                    <div class="col ">
                        <h4>{this.props.hater}</h4>
                    </div>
                    <div class="col ">
                        <h4>{this.props.messages}</h4>
                    </div>
                </div>
            </div>
        );
    }
}

export default function Admin(props)  {

    const [deleteElement, setDeleteElement] = useState(false);
    const [display, setDisplay] = useState(false);

    let user_list = ["johnDoe", "user", "user1", "user2", "user3"]
    let email_list = ["johnDoe@email.com", "user@email.com", "user1@email.com", "user2@email.com", "user3@email.com"]
    let name_list = ["Johnithan Doeythan", "Grand Userton", "Userton First", "Userton Second", "Userton Third"]
    let hater = ["johnDoe", "johnDoe", "johnDoe"]
    let messages = ["Hate speech", "Cringe behavior", "General unpleasantness"]
    let elements = []
    let reportedElements = []

    if(deleteElement) {
        user_list.splice(0,1);
        email_list.splice(0,1);
        name_list.splice(0,1);
        hater.splice(0,3);
        messages.splice(0,3);
    }

    console.log(reportedElements)


    for(var i=0;i<user_list.length;i++){
        // push the component to elements!
       elements.push(<Card user={ user_list[i] } email={email_list[i]} name={name_list[i]} id={i} setDeleteElement={setDeleteElement}/>);
       //console.log(elements)
   }

   for(var i=0;i<hater.length;i++){
    // push the component to elements!
    reportedElements.push(<Reports hater={ hater[i] } messages={messages[i]} id={i}/>);
    //console.log(reportedElements)
}

    const handleToggleUsers = (event) => {
        setDisplay(false)
    }

    const handleToggleReports = (event) => {
        setDisplay(true)
    }
    if(props.user) {
    return (

        <div>
             <button class="btn btn-success my-2 my-sm-0"  type="button" onClick={handleToggleUsers}>User List</button>
             sd
             <button class="btn btn-danger my-2 my-sm-0"  type="button" onClick={handleToggleReports}>Reports</button>
        {!display ?

            <div class="text-white">
                <h2>USER LIST</h2>
                <div class="row">
                    <div class="col border border-white">
                        <h3>Username</h3>
                    </div>
                    <div class="col border border-white">
                        <h3>Email</h3>
                    </div>
                    <div class="col border border-white">
                        <h3>Name</h3>
                    </div>
                </div>
                {elements}
            </div>
            :
            <div className="text-white">
                <h2>REPORTED MESSAGES</h2>
                <div class="row ">
                    <div class="col border border-white">
                        <h3>Username</h3>
                    </div>
                    <div class="col border border-white">
                        <h3>Message</h3>
                    </div>
                </div>
                {reportedElements}
            </div>
        }
        </div>
    )
    }
    else {
        return(
        <AccessForbidden />
        );
}
}