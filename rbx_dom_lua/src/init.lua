local CanonicalInstance = require(script.CanonicalInstance)

return {
	CanonicalProperty = require(script.CanonicalProperty),
	CanonicalInstance = CanonicalInstance,
	PropertySelection = CanonicalInstance.PropertySelection,
}