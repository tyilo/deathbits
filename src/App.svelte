<script>
  let roundsPlayed;
  let players;
  let eyes;
  let eyeSum;
  let diceBits;
  let outcomes;
  let averageProbability;

  class Player {
    constructor() {
      this.name = "";
      this.bitHistory = [...Array(roundsPlayed), players.length];
      this.scoreHistory = Array(roundsPlayed).fill(0);
    }

    get currentBit() {
      return this.bitHistory[this.bitHistory.length - 1];
    }

    set currentBit(bit) {
      this.bitHistory[this.bitHistory.length - 1] = bit;
    }

    get currentScore() {
      return this.scoreHistory[this.scoreHistory.length - 1] || 0;
    }
  }

  function sum(arr) {
    return arr.reduce((a, b) => a + b, 0);
  }

  function getBit(n, i) {
    return (n >> i) & 1;
  }

  function updateWidth(e) {
    const s = e.target.value;
    const span = document.createElement("span");
    span.textContent = s;
    document.body.appendChild(span);
    const width = span.offsetWidth;
    document.body.removeChild(span);
    e.target.style.minWidth = `${width + 30}px`;
  }

  function addPlayer() {
    players = [...players, new Player()];
  }

  function resetGame() {
    roundsPlayed = 0;
    players = [];
    eyes = [];
    eyeSum = null;
    diceBits = "";

    for (let i = 0; i < 2; i++) {
      addPlayer();
    }
  }

  function permutations(n) {
    if (n === 0) return [[]];

    const res = [];
    for (const p of permutations(n - 1)) {
      for (let i = 0; i < n; i++) {
        const p2 = [...p];
        p2.splice(i, 0, n - 1);
        res.push(p2);
      }
    }
    return res;
  }

  function countConsecutives(arr) {
    const res = [];
    let prev = null;
    for (const el of arr) {
      if (el !== prev) {
        res.push([0, el]);
      }
      res[res.length - 1][0]++;
      prev = el;
    }
    return res;
  }

  function assignBits() {
    const scores = players.map(p => p.currentScore);
    const sortedScores = [...scores].sort();
    const sortedOutcomes = [...outcomes].sort();

    function scoreAssignment(p) {
      const score = [];
      for (let i = 0; i < players.length; i++) {
        score.push(outcomes[p[i]]);
      }
      return score;
    }

    function cmpNumbers(n1, n2) {
      if (n1 === n2) return 0;
      if (n1 < n2) return -1;
      return 1;
    }

    function cmpAssignments(p1, p2) {
      for (let i = 0; i < players.length; i++) {
        for (let j = i + 1; j < players.length; j++) {
          const c1s = cmpNumbers(scores[p1[i]], scores[p2[j]]);
          const c1o = outcomes[p1[i]] < outcomes[p2[j]];
        }
      }
    }

    // TODO
    for (let i = 0; i < players.length; i++) {
      players[i].currentBit = i;
    }
  }

  function removePlayer(i) {
    if (roundsPlayed && !confirm(`Remove player ${i + 1}?`)) return;

    players.splice(i, 1);
    players = players;

    assignBits();
  }

  function dieSymbol(n) {
    return String.fromCharCode(0x2680 - 1 + n);
  }

  function rollDice() {
    eyes = [];
    for (let i = 0; i < dice; i++) {
      eyes.push(Math.floor(Math.random() * 6 + 1));
    }
    eyeSum = sum(eyes);
    diceBits = eyeSum.toString(2).padStart(players.length, "0");

    for (let i = 0; i < players.length; i++) {
      let newScore = players[i].currentScore;
      if (getBit(eyeSum, i)) {
        newScore++;
      }
      players[i].scoreHistory.push(newScore);
      players[i].bitHistory.push(null);
    }

    assignBits();

    players = players;

    roundsPlayed++;
  }

  $: dice = Math.ceil(Math.pow(2, players.length - 1) / 6);

  $: {
    if (dice <= 8) {
      outcomes = Array(players.length).fill(0);
      (function f(vals) {
        if (vals.length === dice) {
          const val = sum(vals);
          for (let i = 0; i < players.length; i++) {
            outcomes[i] += getBit(val, i);
          }
        } else {
          for (let i = 1; i <= 6; i++) {
            f([...vals, i]);
          }
        }
      })([]);
      averageProbability = sum(outcomes) / (players.length * Math.pow(6, dice));
    } else {
      outcomes = Array(players.length).fill(null);
      averageProbability = null;
    }
  }

  resetGame();
</script>

<style>
  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }

  .eyes {
    font-size: 6em;
    min-height: 1.25em;
    line-height: 1.25;
  }

  .bits {
    font-size: 3em;
    min-height: 1.25em;
    line-height: 1.25;
  }

  .playerName {
    width: 100%;
    display: table-cell;
  }

  table {
    margin: auto;
  }

  table,
  tr,
  th,
  td {
    border: 1px solid black;
    border-collapse: collapse;
  }

  .winner * {
    background-color: lightgreen;
  }

  td.number {
    min-width: 1.5em;
  }

  th {
    padding: 0.5em;
  }

  button.big {
    font-size: 1.5em;
    margin: 1em;
  }
</style>

<main>
  <h1>Death Bits</h1>
  <div class="eyes">
    {#if eyes.length}
      {#each eyes as n}{dieSymbol(n)}{/each}
    {:else}
      {#each Array(dice) as _}&#x2610;{/each}
    {/if}
  </div>
  <div class="bits">
    {#if eyeSum}
      {#each diceBits as b}{b}{/each}
      ({eyeSum})
    {/if}
  </div>
  <button on:click={rollDice} class="big">Roll</button>
  <table>
    <thead>
      <tr>
        <th colspan={roundsPlayed + 1}>Bits</th>
        <th>Player</th>
        {#if roundsPlayed}
          <th colspan={roundsPlayed}>Scores</th>
        {/if}
        <th />
      </tr>
    </thead>
    <tbody>
      {#each players as player, i}
        <tr class:winner={getBit(eyeSum, i)}>
          {#each [...player.bitHistory].reverse() as bit}
            <td class="number">{bit === undefined ? '-' : bit}</td>
          {/each}
          <td>
            <input
              bind:value={player.name}
              placeholder="Player {i + 1}"
              on:change={updateWidth}
              on:keydown={updateWidth}
              on:keyup={updateWidth}
              class="playerName"
              size="5" />
          </td>
          {#each player.scoreHistory as score}
            <td class="number">{score}</td>
          {/each}
          <td>
            <button
              on:click={() => removePlayer(i)}
              disabled={players.length === 1}
              type="checkbox">
              x
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
  <button on:click={addPlayer} class="big">Add player</button>
  <button on:click={resetGame} class="big">Reset game</button>
  <h2>Probabilities</h2>
  <table>
    <thead>
      <tr>
        <th>Bit</th>
        <th>Probability</th>
      </tr>
    </thead>
    <tbody>
      {#each outcomes as o, i}
        <tr>
          <td>{i}</td>
          <td>
            {#if o !== null}
              <sup>{o}</sup>
              &frasl;
              <sub>{Math.pow(6, dice)}</sub>
              &approx; {(o / Math.pow(6, dice)).toFixed(2)}
            {:else}?{/if}
          </td>
        </tr>
      {/each}
      <tr>
        <th>Avg.</th>
        <td>
          {#if averageProbability !== null}
            {averageProbability.toFixed(2)}
          {:else}?{/if}
        </td>
      </tr>
    </tbody>
  </table>
</main>
