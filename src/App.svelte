<script lang="ts">

import Calc from "./lib/calc.svelte";

let tooltips = [
    { id: 1, text: 'Первый элемент' },
    { id: 2, text: 'Второй элемент' },
    { id: 3, text: 'Третий элемент' }
  ];

let answer = ""
import { invoke } from "@tauri-apps/api/tauri"
  
let name = "";
let greetMsg = ""

async function calculate(){
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg = await invoke("greet", { name })
}

async function callExport() {
  await invoke('export');
}


</script>

<div class="navbar">
  <div class="dropdown">
    <button class="dropbtn">Опции</button>
    <div class="dropdown-content">
      <button class="dropdown-button" on:click={callExport}>Экспорт</button>
      <!--<button class="dropdown-button">Экспорт</button>-->
      <!--<button class="dropdown-button">Экспорт</button>-->
    </div>
  </div>
</div>

<h2 class="centered">Рассчёт эффекта Джоуля-Томпсона</h2>

<div class="container">
</div>

<main class="container">

  <div class="svg-element">
    <div class="rectangle-1">
      <h4>
        P1, T1
      </h4>
      <svg width="250" height="70" viewBox="0 0 250 70" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect width="250" height="70" fill="#1E1E1E"/>
        <rect x="0.5" y="0.5" width="250" height="70" fill="black" stroke="white"/>
        <path d="M-60 86V-17L-30 34.5L0 -17V86L-30 34.5L-60 86Z" fill="black" stroke="white"/>
      </svg>
    </div>
    
    <div class="rectangle-2">
      <h4>
         Редукция
      </h4>
      <svg width="60" height="110" viewBox="0 0 60 131" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M31.75 65.25L1 0.5H62.5L31.75 65.25L62.5 130H1L31.75 65.25Z" fill="black"/>
        <path d="M1 0.5L62.5 130H1L62.5 0.5L1 0.5Z" stroke="white"/>
      </svg>
    </div>
    
    <div class="rectangle-3">
      <h4>
        P2, T2
      </h4>
      <svg width="250" height="70" viewBox="0 0 250 70" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect width="250" height="70" fill="#1E1E1E"/>
        <rect x="0.5" y="0.5" width="250" height="70" fill="black" stroke="white"/>
        <path d="M250 86V-17L280 34.5L310 -17V86L280 34.5L250 86Z" fill="black"/>
        <path d="M250 -17V86L310 -17V86L250 -17Z" stroke="white"/>
      </svg>
    </div>
  </div>


  <div class="input-calc">
    <Calc>

    </Calc>
  </div>
  
 
  <div class="answ">

  </div>
  
</main>
