# RBXM Format

| Overall Structure |
| ----- |
| [Header](#header) |
| Instance class data |
| Instance property data |
| Instance parent data |
| Footer |

## Header
<table>
	<tr>
		<th width="40">0</th>
		<th width="40">1</th>
		<th width="40">2</th>
		<th width="40">3</th>
		<th width="40">4</th>
		<th width="40">5</th>
		<th width="40">6</th>
		<th width="40">7</th>
		<th width="40">8</th>
		<th width="40">9</th>
		<th width="40">10</th>
		<th width="40">11</th>
		<th width="40">12</th>
		<th width="40">13</th>
		<th width="40">14</th>
		<th width="40">15</th>
	</tr>
	<tr>
		<td colspan="16">Magic number</td>
	</tr>
	<tr>
		<td colspan="4">Number of classes</td>
		<td colspan="4">Number of instances</td>
		<td colspan="8">Zero</td>
	</tr>
</table>