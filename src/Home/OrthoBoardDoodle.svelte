<script lang="ts">
  const rowCount = 4
  const columnCount = 16

  const randomInt = (max: number) => () => {
    return Math.floor(Math.random() * (max + 1))
  }

  const buildRow = (columnCount: number) => (): number[] => {
    return new Array(columnCount).fill(0).map(randomInt(100))
  }

  let rows = new Array(rowCount).fill(0).map(buildRow(columnCount))

  const handleOnClickCell = (i: number, j: number) => () => {
    rows = rows.map((row, rowIdx) => {
      return row.map((col, colIdx) => {
        return (rowIdx === i || colIdx === j) ? col + 1 : col
      })
    })
  }

  const isModOf =
    (...moduli: number[]) =>
    (value: number): boolean => {
      return moduli.some((mod) => value % mod === 0)
    }

  const determineColor = (value: number) => {
    const dimmerOnHover = "hover:brightness-95"
    const brighterOnHover = "hover:brightness-150"

    const white = "bg-white-300"
    const gray = "bg-gray-800"
    const purple = "bg-purple-300"
    const blue = "bg-blue-300"
    const green = "bg-green-300"

    const whiteStyle = `${white} hover:bg-gray-100`
    const grayStyle = `${gray} ${brighterOnHover}`
    const purpleStyle = `${purple} ${dimmerOnHover}`
    const blueStyle = `${blue} ${dimmerOnHover}`
    const greenStyle = `${green} ${dimmerOnHover}`

    if (isModOf(2, 5)(value)) {
      return whiteStyle
    } else if (isModOf(23)(value)) {
      return greenStyle
    } else if (isModOf(49)(value)) {
      return blueStyle
    } else if (isModOf(17)(value)) {
      return purpleStyle
    } else {
      return grayStyle
    }
  }
</script>

<div class="aspect-w-4 aspect-h-16 sm:aspect-h-4 sm:aspect-w-16">
  <div class="flex flex-row sm:flex-col">
    {#each rows as row, i}
      <div class="flex flex-col flex-1 sm:flex-row">
        {#each row as col, j}
          <div
            class="cursor-pointer flex-1 filter {determineColor(col)}"
            on:click={handleOnClickCell(i, j)}
          />
        {/each}
      </div>
    {/each}
  </div>
</div>
