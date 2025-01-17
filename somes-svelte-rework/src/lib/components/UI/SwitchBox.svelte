<script lang="ts">
    export let leftText = "2D";
    export let rightText = "3D";
    export let checked = false;
</script>

<label class="toggleSwitch nolabel">
    <input type="checkbox" bind:checked />
    <a hidden ></a>
    <span>
        <span class="left-span">{leftText}</span>
        <span class="right-span">{rightText}</span>
    </span>											
</label>

<style>
/*  Toggle Switch  */

.toggleSwitch span span {
	display: none;
}  
  
.toggleSwitch {
	display: inline-block;
	height: 18px;
	position: relative;
	overflow: visible;
	padding: 0;
	cursor: pointer;
	width: 100px;
	background-color: #fafafa;
	border: 1px solid #ccc;
	border-radius:5px;
	height:34px;
}
.toggleSwitch * {
	-webkit-box-sizing: border-box;
	-moz-box-sizing: border-box;
	box-sizing: border-box;
}
.toggleSwitch > span {
	line-height: 20px;
	height: 20px;
	vertical-align: middle;
}
.toggleSwitch input:focus ~ a {
	outline: none;
}

.toggleSwitch input {
	position: absolute;
	opacity: 0;
	z-index: 5;
}
.toggleSwitch > span {
	position: absolute;
	left: 0;
	width: calc(100% - 6px);
	margin: 0;
	text-align: left;
	white-space: nowrap;
  margin:0 3px;
}
.toggleSwitch > span span {
	position: absolute;
	top: 0;
	left: 0;
	z-index: 5;
	display: block;
	width: 50%;
	margin-left: 50px;
	text-align: left;
	font-size: 0.9em;
	width: auto;
	left: 0;
	top: -1px;
	opacity: 1;
	width:40%;
	text-align: center;
  line-height:34px;
}
.toggleSwitch a {
	position: absolute;
	right: 50%;
	z-index: 4;
	display: block;
	top: 3px;
	bottom: 3px;
	padding: 0;
	left: 3px;
	width: 50%;
	background-color: rgb(var(--color-secondary-300));
	border-radius: 4px;
	-webkit-transition: all 0.2s ease-out;
	-moz-transition: all 0.2s ease-out;
	transition: all 0.2s ease-out;
	box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}
.toggleSwitch > span span:first-of-type {
	color: #FFF;
	opacity: 1;
	left: 0;
	margin: 0;
    width: 50%;
}
.toggleSwitch > span span:last-of-type {
	left:auto;
	right:0;
	color: #999;
	margin: 0;
    width: 50%;
}
.toggleSwitch > span:before {
	content: '';
	display: block;
	width: 100%;
	height: 100%;
	position: absolute;
	left: 0;
	top: -2px;
	border-radius: 30px;
	-webkit-transition: all 0.2s ease-out;
	-moz-transition: all 0.2s ease-out;
	transition: all 0.2s ease-out;
}
.toggleSwitch input:checked ~ a {
	left: calc(50% - 3px);
}

.toggleSwitch input:checked ~ span span:first-of-type {
	left:0;
	color:#999;
}
.toggleSwitch input:checked ~ span span:last-of-type {
	color:#FFF;
}
</style>