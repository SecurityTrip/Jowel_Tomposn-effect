<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from 'svelte';

    const optionsP = [
    { id: 1, label: 'Па' },
    { id: 2, label: 'Бар' },
    { id: 3, label: 'кПа' },
    { id: 4, label: 'МПа' }
    ];

    const optionsT = [
    { id: 1, label: 'K' },
    { id: 2, label: 'C' },
    { id: 3, label: 'F' }
    ];

    let formData = {
      field1: optionsP[0].id,
      field2: optionsP[0].id,
      field3: optionsT[0].id,
      field4: optionsT[0].id
    };

    

    function convertToKelvin(temperature: number, selectedOption: number): number {
    switch (selectedOption) {
      case 1:
        return temperature;
      case 2:
        return temperature + 273.15;
      case 3:
        return (temperature + 459.67) * (5 / 9);
      default:
        throw new Error('Invalid option');
    }
  }

  function convertToPascal(value: number, selectedOption: number): number {
    switch (selectedOption) {
      case 1:
        return value;
      case 2:
        return value * 100000;
      case 3:
        return value * 1000;
      case 4:
        return value * 1000000;
      default:
        throw new Error('Invalid option');
    }
  }


  

  
    async function calculate(){
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      
      //console.log(formData.field1);

      let tmp1 = (document.getElementById('pressure1') as HTMLInputElement).value;
      let tmp2 = (document.getElementById('pressure2') as HTMLInputElement).value;
      let tmp3 = (document.getElementById('temperature1') as HTMLInputElement).value;
      let tmp4 = (document.getElementById('temperature2') as HTMLInputElement).value;


      let p1: number = +tmp1;
      let p2: number = +tmp2;
      let t1: number = +tmp3;
      let t2: number = +tmp4;

      console.log(p1);
      //console.log(p2);
      //console.log(t1);
      //console.log(t2);

      const convertedP1 = convertToPascal(p1, formData.field1);
      p1 = convertedP1;
      const convertedP2 = convertToPascal(p2, formData.field2);
      p2 = convertedP2;
      const convertedT1 = convertToKelvin(t1, formData.field3);
      t1 = convertedT1;
      const convertedT2 = convertToKelvin(t2, formData.field4);
      t2 = convertedT2;

      console.log(p1);
      //console.log(p2);
      //console.log(t1);
      //console.log(t2);

      await invoke("run_calculation", {p1, p2, t1, t2});
      
    }



    
  </script>
  
  <div class = "row">
    <form on:submit|preventDefault={calculate}>
      <div class="column">
        <div class="form-row">
          <label for="field1">P1:</label>
          <input type="number" step="any" id="pressure1" placeholder="Введите давление"/>
          <select bind:value={formData.field1}>
            {#each optionsP as option}
              <option value={option.id}>{option.label}</option>
            {/each}
          </select>
        </div>

        <div class="form-row">
          <label for="field3">T1:</label>
          <input type="number" step="any" id="temperature1" placeholder="Введите температуру"/>
          <select bind:value={formData.field3}>
            {#each optionsT as option}
              <option value={option.id}>{option.label}</option>
            {/each}
          </select>
        </div>
    
        
      </div>
    
      <div class="column">
        <div class="form-row">
          <label for="field2">P2:</label>
          <input type="number" step="any" id="pressure2" placeholder="Введите давление"/>
          <select bind:value={formData.field2}>
            {#each optionsP as option}
              <option value={option.id}>{option.label}</option>
            {/each}
          </select>
        </div>
    
        <div class="form-row">
          <label for="field4">T2:</label>
          <input type="number" step="any" id="temperature2" placeholder="Введите температуру"/>
          <select bind:value={formData.field4}>
            {#each optionsT as option}
              <option value={option.id}>{option.label}</option>
            {/each}
          </select>
          </div>
      </div>
    
      <button type="submit">Отправить</button>
    </form>
  </div>