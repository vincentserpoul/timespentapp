<script lang="ts">
  import stringToColor from "./stringToColor";

  export let activity: string;
  export let selected: boolean;
  const color = stringToColor(activity);
  const opacity = selected ? "0.9" : "0.4";
</script>

<button class="button" href="#" style:background-color={color} style:opacity>
  {activity}
</button>

<style>
  .button {
    border-radius: 1em;
    color: #121212;
    text-transform: lowercase;
    padding: 16px 32px;
    text-align: center;
    transition: 0.6s;
    cursor: pointer;
  }
</style>
