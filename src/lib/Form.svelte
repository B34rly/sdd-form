<script lang>
	import { invoke } from '@tauri-apps/api/tauri';
	import { message } from '@tauri-apps/api/dialog';

	let firstName = '';
	let lastName = '';
	let dob = '';
	let homePhone = '';
	let mobilePhone = '';
	let address = '';

	let msg = {
		first_name: '',
		last_name: '',
		dob: '',
		home_phone: '',
		mobile_phone: '',
		address: ''
	};
	async function submit() {
		//lastName_msg = await invoke('greet', { name: firstName })
		msg = {
			first_name: '',
			last_name: '',
			dob: '',
			home_phone: '',
			mobile_phone: '',
			address: ''
		};

		let errors = await invoke('check_form_data', {
			firstName,
			lastName,
			dob,
			homePhone,
			mobilePhone,
			address
		});
		let error_amount = 0;
		for (let error_field in errors) {
			error_amount++;
            console.log(error_amount)
			msg[error_field] = errors[error_field];
		}

		if (error_amount > 0) {
			let error_message = '';
			let error_title = '';
			if (error_amount == 1) {
				error_title += 'An error was found within your form';
				error_message = error_title + ':\n';
				for (let error_field in errors) {
					let error_topic = error_field.split('_');
					for (let i = 0; i < error_topic.length; i++) {
						error_topic[i] = error_topic[i][0].toUpperCase() + error_topic[i].substring(1);
					}
					error_message += "      " + error_topic.join(" ") +": " + errors[error_field] + "\n";
				}
			} else {
				error_title += 'Multiple errors were found within your form';
				error_message = error_title + ':\n';
				for (let error_field in errors) {
					let error_topic = error_field.split('_');
					for (let i = 0; i < error_topic.length; i++) {
						error_topic[i] = error_topic[i][0].toUpperCase() + error_topic[i].substring(1);
					}
					error_message += "      " + error_topic.join(" ") +": " + errors[error_field]  + "\n";
				}
			}
            error_message += "Please resolve and resubmit."
			message(error_message, { title: error_title, type: 'error' });
		}
	}
</script>

<div>
	<input id="firstName-input" placeholder="First Name" bind:value={firstName} />
	<p>{msg.first_name}</p>
	<input id="lastName-input" placeholder="Last Name" bind:value={lastName} />
	<p>{msg.last_name}</p>
	<input id="dob-input" placeholder="Date of Birth (dd/mm/YYYY)" bind:value={dob} />
	<p>{msg.dob}</p>
	<input
		id="homePhone-input"
		placeholder="Home Phone (02&nbsp;5550&nbsp;4321)"
		bind:value={homePhone}
	/>
	<p>{msg.home_phone}</p>
	<input
		id="mobilePhone-input"
		placeholder="Mobile Phone (0491&nbsp;570&nbsp;159)"
		bind:value={mobilePhone}
	/>
	<p>{msg.mobile_phone}</p>
	<input id="address-input" placeholder="Address" bind:value={address} />
	<p>{msg.address}</p>
	<button on:click={submit}>Submit</button>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		flex-wrap: wrap;
		padding: 2em;
		align-items: center;
		background-color: black;
		background-image: linear-gradient(45deg, rgba(197, 255, 5, 0.5), rgba(68, 255, 0, 0.5));
	}
	input {
		width: 40vw;
		height: 1.5em;
		padding: 0.5em;
		font-size: 1em;
		border-radius: 0.5rem;
		border-style: solid;
		border-color: black;
		color: white;
		background-color: rgb(50, 50, 50);
	}

	p {
		color: red;
		font-size: smaller;
		margin: 0.25em;
		font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
	}
</style>
