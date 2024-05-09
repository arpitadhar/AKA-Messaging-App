import * as React from "react";
import { useEffect, useState } from "react";
import { Link, Navigate, useNavigate } from "react-router-dom";
import AccessForbidden from "../AccessForbidden/AccessForbidden";
import "./UserProfile.css";


//Changes array buffer in posts response to base64 to display
export const toBase64 = function (arr) {
    //Changes ArrayBuffer to base 64
    const baseSource = btoa(
      arr?.reduce((data, byte) => data + String.fromCharCode(byte), ""),
    );
    //Takes base64 string and concats to get right source for image
    const source = `data:image/jpeg;base64,${baseSource}`;
    return source;
    };

export function ProfileImage(props) {
    return (
      <div className="image-container">
        <div className="image-inner-container">
             <UserImage
                  source={toBase64(e?.photo?.data)}
            /> 
        </div>
      </div>
    );
  }

export default function UserProfile(props)  {
    //Hacky way of implementation, lift this props to App component
    const [firstName, setFirstName] = useState("");
    const [lastName, setLastName] = useState("");
    const [oldPassword, setOldPassword] = useState("")
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");


      //Updates image when changes happen
  const imageChange = (e) => {
    if (e.target.files && e.target.files.length > 0) {
      setSelectedImage(e.target.files[0]);
      setImageError({});
    }
  };

  //Removes image preview if clicked on
  const removeSelectedImage = () => {
    setSelectedImage();
  };

  // open file explorer/finder to select image
  const inputRef = useRef(null);

  const handleClick = () => {
    inputRef.current.click();
  };

  //Updates changes for file values
  const handleFileChange = (e) => {
    const fileObj = e.target.files && e.target.files[0];
    if (!fileObj) {
      return;
    }
  };
  //Submit function that is called when user submits form
  const handleOnSubmit = () => {
    //Validation before sending to backend
    if (selectedImage === undefined) {
      setImageError({ image: "Select an Image" });
      return;
    }
  }

    //Sends request to make a post 
    const uploadImage = async (data) => {
        setIsLoading(true);
        try {
          const getData = await ApiClient.uploadImage(data);

        } catch (err) {
          setError(err);
          console.error(err);
        }
        setIsLoading(false);
      };

    //Sends request to make a post 
    const getImage = async (data) => {
        setIsLoading(true);
        try {
          const getData = await ApiClient.getImage(data);

        } catch (err) {
          setError(err);
          console.error(err);
        }
        setIsLoading(false);
      };
    //TODO
    //I really don't wanna add another input box, didn't realize that there was no username change going on until it was too late
    //TODO

    //const handleUsernameChange = (event) => {
    //    setUsername(event.target.value);
    //}
    
    const handleFirstNameChange = (event) => {
        setFirstName(event.target.value);
    }

    const handleLastNameChange = (event) => {
        setLastName(event.target.value);
    }

    const handleOldPasswordChange = (event) => {
        setOldPassword(event.target.value);
    }

    const handlePasswordChange = (event) => {
        setPassword(event.target.value);
    }

    const handleConfirmPasswordChange = (event) => {
        setConfirmPassword(event.target.value);
    }

    //const handleSubmit = (event) => {
    //    event.preventDefault();
//
    //    document.getElementById('invalid-0').classList.add("d-none");
    //    document.getElementById('invalid-1').classList.add("d-none");
    //
    //    const oldPasswordBox = document.getElementById('oldPassword');
    //    const confirmPasswordBox = document.getElementById('confirmPassword');
//
    //    if(password) {
    //        if(oldPassword){
    //            //send to backend to check if correct
    //            //if true continue, if false throw error message
    //            return;
    //        } else {
    //            oldPasswordBox.classList.add('border-danger');
    //            document.getElementById('invalid-0').classList.remove("d-none");
    //        }
    //        if(password == confirmPassword) {
    //            //send to backend to replace current password
    //            return;
    //        } else {
    //            confirmPasswordBox.classList.add('border-danger');
    //            document.getElementById('invalid-1').classList.remove("d-none");
    //        }
//
    //    }
        
    //}

    const UserProfileImage = getImage()
    
    
    if (props.user) {
        return (

            <div class="container text-white">
            <div class="row">
            		<div class="col-12">
            			<div class="my-5">
            				<h3>JohnDoe's Profile</h3>
                            <ProfileImage
                            source={toBase64(UserProfileImage?.photo?.data)}
                            /> 
            				<hr/>
            			</div>
            			<form class="file-upload">
            				<div class="row mb-5 gx-5">
            					<div class="col-xxl-8 mb-5 mb-xxl-0">
            						<div class="bg-secondary-soft px-4 py-5 rounded">
            							<div class="row g-3">
            								<h4 class="mb-4 mt-0">Personal Info</h4>
            								<div class="col-md-6">
            									<label class="form-label">First Name *</label>
            									<input type="text" class="form-control" placeholder="John" aria-label="First name" onChange={handleFirstNameChange}/>
            								</div>
            								<div class="col-md-6">
            									<label class="form-label">Last Name *</label>
            									<input type="text" class="form-control" placeholder="Doe" aria-label="Last name" onChange={handleLastNameChange}/>
            								</div>
            							</div> 
            						</div>
            					</div>
            					<div class="col-xxl-4">
            						<div class="bg-secondary-soft px-4 py-5 rounded">
            							<div class="row g-3">
            								<h4 class="mb-4 mt-0">Upload your profile photo</h4>
            								<div class="text-center">
            									<div class="square position-relative display-2 mb-3">
            										<i class="fas fa-fw fa-user position-absolute top-50 start-50 translate-middle text-secondary"></i>
            									</div>
            									<input type="file" id="customFile" name="file" hidden=""/>
            									<label class="btn btn-success btn-block" for="customFile">Upload</label>
            									<button type="button" class="btn btn-danger">Remove</button>
            									<p class="text-muted mt-3 mb-0"><span class="me-1">Note:</span>Minimum size 300px x 300px</p>
            								</div>
            							</div>
            						</div>
            					</div>
            				</div> 

            				<div class="row mb-5 gx-5">
            					<div class="bg-secondary-soft px-4 py-5 rounded">
            						<div class="row g-3">
            							<h4 class="my-4">Change Password</h4>
            							<div class="col-md-6">
            								<label for="oldPassword" class="form-label">Old password *</label>
            								<input type="password" class="form-control" id="oldPassword" onChange={handleOldPasswordChange}/>
                                            <div id="invalid-0" class="text-danger d-none">
                                            Please enter your password
                                            </div>
            							</div>
            							<div class="col-md-6">
            								<label for="newPassword" class="form-label">New password *</label>
            								<input type="password" class="form-control" id="newPassword" onChange={handlePasswordChange}/>
            							</div>
            							<div class="col-md-12">
            								<label for="confirmPassword" class="form-label">Confirm Password *</label>
            								<input type="password" class="form-control" id="confirmPassword" onChange={handleConfirmPasswordChange}/>
                                            <div id="invalid-1" class="text-danger d-none">
                                            Passwords do not match
                                            </div>
            							</div>
            						</div>
            					</div>
            				</div>

            				<div class="gap-3 d-md-flex text-center">
            					<button type="button" class="btn btn-danger btn-lg">Delete profile</button>
            					<button type="button" class="btn btn-primary btn-lg" onClick={uploadImage}>Update profile</button>
            				</div>
            			</form> 
            		</div>
            	</div>
            </div>
        );
    }
        else {
            return(
            <AccessForbidden />
            );
    }
}