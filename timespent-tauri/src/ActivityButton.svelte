<script lang="ts">
  import stringToColor from "./stringToColor";

  export let activity: string;
  const color = stringToColor(activity);
</script>

<a class="pure-button activity-button" href="#" style:background-color={color}
  >{activity}</a
>

<style>
  a.activity-button {
    color: white;
    font-family: "Comic Sans MS", cursive;
    font-weight: bold;
  }
</style>
