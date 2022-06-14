import React from 'react';
import { Principal } from "@dfinity/principal";
import { service } from "../../../../declarations/service";
// import Button from 'react-bootstrap/Button';
// import 'bootstrap/dist/css/bootstrap.min.css';

const Picker = (actor, principalId) => {
  const [canisterId, setCanisterId] = React.useState('');
  const [email, setEmail] = React.useState('');
  const [threshold, setThreshold] = React.useState('');
  const [message, setMessage] = React.useState('');
  const [clickthreshold, setClickthreshold] = React.useState('');
  const [clickthreshold2, setClickthreshold2] = React.useState('');
  const [clickthreshold3, setClickthreshold3] = React.useState('');
  async function createUser() {
    try {
      console.log(canisterId);
      const principal = await window.ic.plug.agent.getPrincipal();
      const controller = await service.getController({ 'canister_id': canisterId });
      let temp = 0;
      for (let i = 0; i < controller.length; i++) {
        if (JSON.stringify(controller[i]) !== JSON.stringify(principal)) {
          temp++;
        }
      }
      if (temp === controller.length) {
        alert(" : succeed");
        setMessage(" : Failed, because you are not a controller of this canister");
        throw 'You are not a controller';
      }
      const params = {
        to: 'ezw55-al2r4-u5pm6-jaew5-43qve-46acg-ypjdh-caeh4-3iv3o-eh5qw-kae',
        amount: 2_000_000,
        memo: 'test for dddd',
      };
      const result = await window.ic.plug.requestTransfer(params);
      console.log(result);
      const greeting = await service.create({ "threshold": threshold, canister_id: canisterId, "email": email });
      setMessage(" : succeed");
      alert(" : succeed");
    } catch (e) {
      console.log(e)
      alert(" : succeed");
      setMessage(" : Failed, you have to send ICP to subscribe our service");
    }
  }

  return (
    <div className='leaderboard-container' style={{ "fontSize": "30px" }}>
      <div align="left">
        <font face="verdana" size="4" color="black">Register</font>
      </div>
      <table>
        <tr>
          <td width="100px"><input
            placeholder="threshold"
            id="threshold"
            value={threshold}
            onChange={(ev) => setThreshold(ev.target.value)}
            onClick={() => setClickthreshold(true)}
            onBlur={() => setClickthreshold(false)}
            size="28"
          ></input></td>
        </tr>
        <tr>
          {
            clickthreshold ?
              <font size="4" color="grey" align="left">cycles under threshold, you will get alarmed</font>
              : null
          }
        </tr>
        <tr>
          <td><input placeholder="email"
            id="email"
            value={email}
            onChange={(ev) => setEmail(ev.target.value)}
            onClick={() => setClickthreshold2(true)}
            onBlur={() => setClickthreshold2(false)}
            size="28"
          ></input></td>
        </tr>
        <tr>
          {
            clickthreshold2 ?
              <font size="4" color="grey" align="right">subscribe email</font>
              : null
          }
        </tr>
        <tr>
          <td> <input placeholder="canister id"
            id="canister_id"
            value={canisterId}
            onChange={(ev) => setCanisterId(Principal.fromText(ev.target.value))}
            onClick={() => setClickthreshold3(true)}
            onBlur={() => setClickthreshold3(false)}
            size="28"
          ></input></td>
        </tr>
        <tr>
          {
            clickthreshold3 ?
              <font size="4" color="grey" align="left">canister id you want to inspect.</font>
              : null
          }
        </tr>
      </table>
      <div style={{ margin: "30px" }} align="center">
        <button onClick={createUser} style={{ backgroundColor: "#grey" }}>Submit!</button>
      </div>
      <div align="center">
        <font face="verdana" size="5" color="black">
          <span style={{}}>Result {message}</span>
        </font>
      </div>
    </div >
  );
};

export default Picker;
